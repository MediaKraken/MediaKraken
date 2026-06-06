#!/usr/bin/env python3
"""
Take CloudNativePG snapshots via TopoLVM VolumeSnapshots and restore clusters from them.

Usage:
    cnpg_snapshot.py create <cluster> [--namespace cnpg-system] [--keep <n>] [--wait]
    cnpg_snapshot.py list  [--namespace cnpg-system] [--cluster <cluster>]
    cnpg_snapshot.py delete <snapshot-name> [--namespace topolvm-system]
    cnpg_snapshot.py status <snapshot-name> [--namespace topolvm-system]
    cnpg_snapshot.py restore <cluster> --from-snapshot <snapshot-name> [--new-cluster <name>] [--namespace cnpg-system]
    cnpg_snapshot.py restore list-snapshots [--cluster <cluster>] [--namespace cnpg-system]

Requires: kubectl (with access to the cluster), Python 3.8+
"""

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from typing import Optional

TOPOLOVM_SNAPSHOT_CLASS = "topolvm-lvmsnapshot"

def run_cmd(args: list[str], check: bool = True) -> subprocess.CompletedProcess:
    """Run a command and return the result."""
    try:
        return subprocess.run(args, capture_output=True, text=True, check=check)
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr.strip()}", file=sys.stderr)
        sys.exit(1)


def get_pvs_for_cluster(cluster: str, namespace: str) -> list[dict]:
    """Get PVCs belonging to a CNPG cluster by label."""
    cmd = [
        "kubectl", "get", "pvc",
        "-n", namespace,
        "-l", f"cnpg.io/cluster={cluster}",
        "-o", "json",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        print(f"Error: Could not find PVCs for cluster '{cluster}' in namespace '{namespace}'", file=sys.stderr)
        sys.exit(1)

    data = json.loads(result.stdout)
    items = data.get("items", [])
    if not items:
        print(f"No PVCs found for cluster '{cluster}' in namespace '{namespace}'", file=sys.stderr)
        sys.exit(1)
    return items


def create_snapshot(
    pvc_name: str,
    namespace: str,
    snapshot_class: str,
    timestamp: str,
) -> str:
    """Create a VolumeSnapshot and return the snapshot name."""
    snapshot_name = f"cnpg-{pvc_name}-{timestamp}"
    snapshot_yaml = {
        "apiVersion": "snapshot.storage.k8s.io/v1",
        "kind": "VolumeSnapshot",
        "metadata": {
            "name": snapshot_name,
            "namespace": namespace,
            "labels": {
                "cnpg.io/cluster": cluster_from_pvc_name(pvc_name),
                "snapshot-type": "cnpg-snapshot",
                "created-at": timestamp,
            },
        },
        "spec": {
            "volumeSnapshotClassName": snapshot_class,
            "source": {
                "persistentVolumeClaimName": pvc_name,
                "apiGroup": "snapshot.storage.k8s.io",
                "kind": "VolumeSnapshotContent",
            },
        },
    }
    cmd = ["kubectl", "apply", "-f", "-"]
    proc = subprocess.run(cmd, input=json.dumps(snapshot_yaml), capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"Error creating snapshot {snapshot_name}:", file=sys.stderr)
        print(proc.stderr.strip(), file=sys.stderr)
        sys.exit(1)
    print(f"Created VolumeSnapshot: {snapshot_name}")
    return snapshot_name


def cluster_from_pvc_name(pvc_name: str) -> str:
    """Extract CNPG cluster name from PVC name.

    CNPG PVC naming: <cluster>-<instance-id>
    e.g. pgcluster-with-metrics-1 -> pgcluster-with-metrics
    """
    parts = pvc_name.rsplit("-", 1)
    return parts[0] if len(parts) > 1 else pvc_name


def wait_for_snapshots(snapshot_names: list[str], namespace: str, timeout: int = 600) -> bool:
    """Wait for VolumeSnapshots to be readyToUse."""
    print(f"Waiting for {len(snapshot_names)} snapshot(s) to be ready (timeout: {timeout}s)...")
    start = datetime.now(timezone.utc)
    names_str = ",".join(snapshot_names)

    while True:
        cmd = [
            "kubectl", "get", "volumesnapshot",
            "-n", namespace,
            names_str,
            "-o", "json",
        ]
        result = run_cmd(cmd, check=False)
        if result.returncode != 0:
            print(f"Error: could not check snapshot status", file=sys.stderr)
            return False

        data = json.loads(result.stdout)
        all_ready = True
        for item in data.get("items", []):
            name = item["metadata"]["name"]
            ready = item.get("status", {}).get("readyToUse", False)
            bound = item.get("status", {}).get("boundVolumeSnapshotContentName", "N/A")
            print(f"  {name}: ready={ready}, content={bound}")
            if not ready:
                all_ready = False

        if all_ready:
            print("All snapshots are ready.")
            return True

        elapsed = (datetime.now(timezone.utc) - start).total_seconds()
        if elapsed >= timeout:
            print(f"Timeout after {timeout}s. Some snapshots may not be ready.", file=sys.stderr)
            return False

        import time
        time.sleep(10)


def list_snapshots(namespace: str, cluster: Optional[str] = None):
    """List VolumeSnapshots, optionally filtered by cluster label."""
    labels = ["snapshot-type=cnpg-snapshot"]
    if cluster:
        labels.append(f"cnpg.io/cluster={cluster}")

    label_selector = ",".join(labels)
    cmd = [
        "kubectl", "get", "volumesnapshot",
        "-n", namespace,
        "-l", label_selector,
        "-o", "custom-columns="
              "NAME:.metadata.name,"
              "CLUSTER:.metadata.labels.cnpg\\.io/cluster,"
              "CREATED:.metadata.labels.created-at,"
              "SOURCE_PVC:.spec.source.persistentVolumeClaimName,"
              "READY:.status.readyToUse,"
              "CONTENT:.status.boundVolumeSnapshotContentName,"
              "AGE:.metadata.creationTimestamp",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        print(f"Error listing snapshots: {result.stderr.strip()}", file=sys.stderr)
        return

    output = result.stdout.strip()
    if not output or "No resources found" in output:
        print("No snapshots found.")
        return
    print(output)


def delete_snapshot(name: str, namespace: str):
    """Delete a VolumeSnapshot."""
    cmd = ["kubectl", "delete", "volumesnapshot", name, "-n", namespace]
    result = run_cmd(cmd)
    print(result.stdout.strip())


def snapshot_status(name: str, namespace: str):
    """Show detailed status of a VolumeSnapshot."""
    cmd = [
        "kubectl", "get", "volumesnapshot", name,
        "-n", namespace,
        "-o", "json",
    ]
    result = run_cmd(cmd)
    data = json.loads(result.stdout)

    meta = data.get("metadata", {})
    status = data.get("status", {})

    print(f"Name:          {meta.get('name', 'N/A')}")
    print(f"Namespace:     {meta.get('namespace', 'N/A')}")
    print(f"Created:       {meta.get('creationTimestamp', 'N/A')}")
    print(f"Ready:         {status.get('readyToUse', False)}")
    print(f"Source PVC:    {data.get('spec', {}).get('source', {}).get('persistentVolumeClaimName', 'N/A')}")
    print(f"Snapshot Class:{data.get('spec', {}).get('volumeSnapshotClassName', 'N/A')}")
    print(f"Content Name:  {status.get('boundVolumeSnapshotContentName', 'N/A')}")
    print(f"Snapshot Size: {status.get('snapshotSize', 'N/A')}")

    if "creationTimestamp" in status:
        print(f"Ready At:      {status['creationTimestamp']}")

    labels = meta.get("labels", {})
    print(f"Cluster:       {labels.get('cnpg.io/cluster', 'N/A')}")
    print(f"Labels:        {json.dumps(labels, indent=2)}")


def cleanup_old_snapshots(namespace: str, cluster: str, keep: int):
    """Delete oldest snapshots, keeping only the most recent <keep> per PVC."""
    cmd = [
        "kubectl", "get", "volumesnapshot",
        "-n", namespace,
        "-l", f"cnpg.io/cluster={cluster},snapshot-type=cnpg-snapshot",
        "-o", "json",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        return

    data = json.loads(result.stdout)
    items = data.get("items", [])
    if not items:
        return

    # Group by source PVC
    by_pvc: dict[str, list[dict]] = {}
    for item in items:
        pvc = item.get("spec", {}).get("source", {}).get("persistentVolumeClaimName", "unknown")
        by_pvc.setdefault(pvc, []).append(item)

    deleted = 0
    for pvc, snaps in by_pvc.items():
        # Sort by creation timestamp (oldest first)
        snaps.sort(key=lambda s: s.get("metadata", {}).get("creationTimestamp", ""))
        to_delete = snaps[:-keep] if len(snaps) > keep else []

        for snap in to_delete:
            name = snap["metadata"]["name"]
            run_cmd(["kubectl", "delete", "volumesnapshot", name, "-n", namespace])
            print(f"Deleted old snapshot: {name}")
            deleted += 1

    if deleted:
        print(f"Cleaned up {deleted} old snapshot(s), keeping {keep} per PVC.")
    else:
        print(f"No snapshots to clean up (keeping {keep} per PVC).")


def generate_timestamp() -> str:
    """Generate a timestamp string suitable for snapshot naming."""
    return datetime.now(timezone.utc).strftime("%Y%m%d%H%M%S")


def get_snapshot_details(snapshot_name: str, namespace: str) -> dict:
    """Get VolumeSnapshot details including source PVC info."""
    cmd = [
        "kubectl", "get", "volumesnapshot", snapshot_name,
        "-n", namespace,
        "-o", "json",
    ]
    result = run_cmd(cmd)
    return json.loads(result.stdout)


def get_snapshot_pvc_names(snapshots: list[dict]) -> dict[str, str]:
    """Map source PVC name -> snapshot name from a list of snapshots taken together."""
    mapping = {}
    for snap in snapshots:
        pvc = snap.get("spec", {}).get("source", {}).get("persistentVolumeClaimName", "")
        if pvc:
            mapping[pvc] = snap["metadata"]["name"]
    return mapping


def get_original_cluster_spec(cluster: str, namespace: str) -> Optional[dict]:
    """Fetch the original cluster spec to use as a template for restore."""
    cmd = [
        "kubectl", "get", "cluster", cluster,
        "-n", namespace,
        "-o", "json",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        return None
    return json.loads(result.stdout)


def create_backup_from_snapshot(
    snapshot_name: str,
    namespace: str,
    backup_name: str,
) -> str:
    """Create a CNPG Backup CR that references a VolumeSnapshot."""
    backup_yaml = {
        "apiVersion": "postgresql.cnpg.io/v1",
        "kind": "Backup",
        "metadata": {
            "name": backup_name,
            "namespace": namespace,
            "labels": {
                "snapshot-source": snapshot_name,
                "created-by": "cnpg_snapshot.py",
            },
        },
        "spec": {
            "cluster": {
                "name": "PLACEHOLDER_CLUSTER",
            },
            "method": "volumeSnapshot",
            "volumeSnapshot": {
                "snapshotName": snapshot_name,
            },
        },
    }
    cmd = ["kubectl", "apply", "-f", "-"]
    proc = subprocess.run(cmd, input=json.dumps(backup_yaml), capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"Error creating Backup '{backup_name}':", file=sys.stderr)
        print(proc.stderr.strip(), file=sys.stderr)
        sys.exit(1)
    print(f"Created Backup: {backup_name}")
    return backup_name


def create_cluster_from_backup(
    new_cluster_name: str,
    backup_name: str,
    original_spec: dict,
    namespace: str,
    instances: int = 1,
) -> str:
    """Create a new CNPG Cluster CR that bootstraps from a Backup (snapshot)."""
    original_spec = original_spec.get("spec", {})

    cluster_yaml = {
        "apiVersion": "postgresql.cnpg.io/v1",
        "kind": "Cluster",
        "metadata": {
            "name": new_cluster_name,
            "namespace": namespace,
            "labels": {
                "restored-from": backup_name,
                "restored-by": "cnpg_snapshot.py",
            },
        },
        "spec": {
            "instances": instances,
            "imageName": original_spec.get("imageName", "ghcr.io/cloudnative-pg/postgis:18.1-3.6.1-202512151010-system-trixie"),
            "primaryUpdateStrategy": "unsupervised",
            "bootstrap": {
                "recovery": {
                    "backup": {
                        "name": backup_name,
                    },
                },
            },
            "enableSuperuserAccess": True,
            "backup": original_spec.get("backup"),
            "postgresql": original_spec.get("postgresql", {}),
            "storage": original_spec.get("storage", {}),
            "resources": original_spec.get("resources", {}),
            "affinity": original_spec.get("affinity", {}),
            "startDelay": original_spec.get("startDelay", 300),
            "stopDelay": original_spec.get("stopDelay", 300),
            "pgbouncer": original_spec.get("pgbouncer"),
        },
    }

    cmd = ["kubectl", "apply", "-f", "-"]
    proc = subprocess.run(cmd, input=json.dumps(cluster_yaml, indent=2), capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"Error creating Cluster '{new_cluster_name}':", file=sys.stderr)
        print(proc.stderr.strip(), file=sys.stderr)
        sys.exit(1)
    print(f"Created Cluster '{new_cluster_name}' bootstrapping from Backup '{backup_name}'")
    return new_cluster_name


def wait_for_cluster_ready(cluster_name: str, namespace: str, timeout: int = 600) -> bool:
    """Wait for a CNPG cluster to become ready."""
    print(f"Waiting for cluster '{cluster_name}' to become ready (timeout: {timeout}s)...")
    start = datetime.now(timezone.utc)

    while True:
        cmd = [
            "kubectl", "get", "cluster", cluster_name,
            "-n", namespace,
            "-o", "json",
        ]
        result = run_cmd(cmd, check=False)
        if result.returncode != 0:
            print(f"Error: could not check cluster status", file=sys.stderr)
            return False

        data = json.loads(result.stdout)
        status = data.get("status", {})
        ready = status.get("ready", False)
        instances = status.get("instances", 0)
        ready_instances = status.get("readyInstances", 0)
        phase = status.get("clusterPhase", "unknown")

        print(f"  Phase: {phase}, Instances: {ready_instances}/{instances}, Ready: {ready}")

        if ready and ready_instances >= 1:
            print(f"Cluster '{cluster_name}' is ready.")
            return True

        elapsed = (datetime.now(timezone.utc) - start).total_seconds()
        if elapsed >= timeout:
            print(f"Timeout after {timeout}s. Cluster may not be fully ready.", file=sys.stderr)
            return False

        import time
        time.sleep(15)


def list_snapshots_for_restore(cluster: Optional[str], namespace: str):
    """List snapshots that can be used for restore, grouped by timestamp."""
    labels = ["snapshot-type=cnpg-snapshot"]
    if cluster:
        labels.append(f"cnpg.io/cluster={cluster}")

    label_selector = ",".join(labels)
    cmd = [
        "kubectl", "get", "volumesnapshot",
        "-n", namespace,
        "-l", label_selector,
        "-o", "json",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        print(f"Error listing snapshots: {result.stderr.strip()}", file=sys.stderr)
        return

    data = json.loads(result.stdout)
    items = data.get("items", [])
    if not items:
        print("No snapshots found for restore.")
        return

    # Group snapshots by their creation timestamp label (snapshots taken together share a timestamp)
    by_ts: dict[str, list[dict]] = {}
    for item in items:
        ts = item.get("metadata", {}).get("labels", {}).get("created-at", "unknown")
        by_ts.setdefault(ts, []).append(item)

    if not by_ts:
        print("No snapshots found for restore.")
        return

    print(f"\n{'Timestamp':<20} {'Cluster':<35} {'Snapshots':<15} {'PVCs':<50} {'Ready'}")
    print("-" * 140)

    for ts in sorted(by_ts.keys(), reverse=True):
        snaps = by_ts[ts]
        cluster_name = snaps[0].get("metadata", {}).get("labels", {}).get("cnpg.io/cluster", "?")
        pvc_names = [s.get("spec", {}).get("source", {}).get("persistentVolumeClaimName", "?") for s in snaps]
        ready_count = sum(1 for s in snaps if s.get("status", {}).get("readyToUse", False))
        pvc_str = ", ".join(pvc_names)

        snap_names = [s["metadata"]["name"] for s in snaps]
        print(f"{ts:<20} {cluster_name:<35} {len(snaps):<15} {pvc_str:<50} {ready_count}/{len(snaps)}")
        for sn in snap_names:
            print(f"  -> {sn}")
        print()


def cmd_restore(args):
    cluster = args.cluster
    namespace = args.namespace
    snapshot_name = args.from_snapshot
    new_cluster_name = args.new_cluster
    instances = args.instances
    wait = args.wait
    dry_run = args.dry_run

    # Get snapshot details
    print(f"Loading snapshot '{snapshot_name}' from namespace '{namespace}'...")
    snap_data = get_snapshot_details(snapshot_name, namespace)
    source_pvc = snap_data.get("spec", {}).get("source", {}).get("persistentVolumeClaimName", "")
    cluster_label = snap_data.get("metadata", {}).get("labels", {}).get("cnpg.io/cluster", cluster)
    ready = snap_data.get("status", {}).get("readyToUse", False)

    print(f"  Source PVC:   {source_pvc}")
    print(f"  Cluster:      {cluster_label}")
    print(f"  Ready:        {ready}")
    if not ready:
        print("  WARNING: Snapshot is not yet readyToUse. Restore may fail.", file=sys.stderr)

    # Get original cluster spec
    print(f"Loading original cluster spec for '{cluster}'...")
    original_spec = get_original_cluster_spec(cluster, namespace)
    if not original_spec:
        print(f"Error: Could not find cluster '{cluster}' in namespace '{namespace}'", file=sys.stderr)
        sys.exit(1)

    # Determine backup name and new cluster name
    base_name = snapshot_name.replace("cnpg-", "")
    backup_name = f"restore-{base_name}"
    if new_cluster_name is None:
        new_cluster_name = f"{base_name}-restore"

    # Dry run: show what would be created
    if dry_run:
        print("\n=== DRY RUN: What would be created ===\n")

        backup_yaml = {
            "apiVersion": "postgresql.cnpg.io/v1",
            "kind": "Backup",
            "metadata": {"name": backup_name, "namespace": namespace},
            "spec": {
                "cluster": {"name": cluster},
                "method": "volumeSnapshot",
                "volumeSnapshot": {"snapshotName": snapshot_name},
            },
        }
        print(f"Backup YAML:\n{json.dumps(backup_yaml, indent=2)}\n")

        cluster_yaml = {
            "apiVersion": "postgresql.cnpg.io/v1",
            "kind": "Cluster",
            "metadata": {
                "name": new_cluster_name,
                "namespace": namespace,
                "labels": {"restored-from": backup_name},
            },
            "spec": {
                "instances": instances,
                "imageName": original_spec.get("imageName", "N/A"),
                "bootstrap": {"recovery": {"backup": {"name": backup_name}}},
                "enableSuperuserAccess": True,
            },
        }
        print(f"Cluster YAML:\n{json.dumps(cluster_yaml, indent=2)}")
        return

    # Create backup CR
    print(f"\nCreating Backup '{backup_name}'...")
    create_backup_from_snapshot(snapshot_name, namespace, backup_name)

    # Wait for backup to be ready
    print(f"Waiting for Backup '{backup_name}' to be ready...")
    start = datetime.now(timezone.utc)
    while True:
        cmd = [
            "kubectl", "get", "backup", backup_name,
            "-n", namespace,
            "-o", "json",
        ]
        result = run_cmd(cmd, check=False)
        if result.returncode != 0:
            print(f"Error checking backup status", file=sys.stderr)
            return False

        backup_data = json.loads(result.stdout)
        phase = backup_data.get("status", {}).get("phase", "unknown")
        print(f"  Backup phase: {phase}")

        if phase in ["ready", "completed"]:
            print(f"Backup '{backup_name}' is ready.")
            break
        elif phase in ["failed", "error"]:
            print(f"Backup '{backup_name}' failed with phase: {phase}", file=sys.stderr)
            sys.exit(1)

        elapsed = (datetime.now(timezone.utc) - start).total_seconds()
        if elapsed >= 300:
            print("Timeout waiting for backup.", file=sys.stderr)
            sys.exit(1)

        import time
        time.sleep(10)

    # Create new cluster from backup
    print(f"\nCreating Cluster '{new_cluster_name}' from Backup '{backup_name}'...")
    new_cluster = create_cluster_from_backup(new_cluster_name, backup_name, original_spec, namespace, instances)

    if wait:
        ok = wait_for_cluster_ready(new_cluster_name, namespace)
        if not ok:
            print("Cluster may not be fully ready. Check with:", file=sys.stderr)
            print(f"  kubectl get cluster {new_cluster_name} -n {namespace}", file=sys.stderr)
            sys.exit(2)

    print(f"\nRestore complete!")
    print(f"  Backup:  {backup_name}")
    print(f"  Cluster: {new_cluster_name}")
    print(f"\nTo connect to the restored cluster:")
    print(f"  kubectl get secret {new_cluster_name}-app-user -n {namespace} -o jsonpath='{{.data.username}}' | base64 -d")
    print(f"  kubectl get secret {new_cluster_name}-app-user -n {namespace} -o jsonpath='{{.data.password}}' | base64 -d")
    print(f"  Host: {new_cluster_name}-pgbouncer-rw.{namespace}.svc.cluster.local")


def cmd_create(args):
    cluster = args.cluster
    namespace = args.namespace
    snapshot_class = args.snapshot_class
    keep = args.keep
    wait = args.wait
    timeout = args.timeout

    timestamp = generate_timestamp()
    pvcs = get_pvs_for_cluster(cluster, namespace)

    print(f"Found {len(pvcs)} PVC(s) for cluster '{cluster}':")
    for pvc in pvcs:
        name = pvc["metadata"]["name"]
        size = pvc.get("spec", {}).get("resources", {}).get("requests", {}).get("storage", "?")
        sc = pvc.get("spec", {}).get("storageClassName", "?")
        phase = pvc.get("status", {}).get("phase", "?")
        print(f"  {name}  ({size}, storageClass={sc}, phase={phase})")

    snapshot_names = []
    for pvc in pvcs:
        name = pvc["metadata"]["name"]
        snap_name = create_snapshot(name, namespace, snapshot_class, timestamp)
        snapshot_names.append(snap_name)

    if wait:
        ok = wait_for_snapshots(snapshot_names, namespace, timeout)
        if not ok:
            sys.exit(2)

    if keep is not None:
        cleanup_old_snapshots(namespace, cluster, keep)


def main():
    parser = argparse.ArgumentParser(
        description="Take CloudNativePG snapshots via TopoLVM VolumeSnapshots",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # create
    p_create = subparsers.add_parser("create", help="Create snapshots for all PVCs of a CNPG cluster")
    p_create.add_argument("cluster", help="CNPG cluster name (e.g. pgcluster-with-metrics)")
    p_create.add_argument("--namespace", "-n", default="cnpg-system", help="Namespace of the CNPG cluster (default: cnpg-system)")
    p_create.add_argument("--snapshot-class", default=TOPOLOVM_SNAPSHOT_CLASS, help=f"TopoLVM snapshot class (default: {TOPOLOVM_SNAPSHOT_CLASS})")
    p_create.add_argument("--keep", type=int, default=None, help="Number of recent snapshots to keep per PVC (deletes older ones)")
    p_create.add_argument("--wait", "-w", action="store_true", help="Wait for snapshots to become ready")
    p_create.add_argument("--timeout", type=int, default=600, help="Timeout in seconds for --wait (default: 600)")

    # list
    p_list = subparsers.add_parser("list", help="List CNPG snapshots")
    p_list.add_argument("--namespace", "-n", default="cnpg-system", help="Namespace to search (default: cnpg-system)")
    p_list.add_argument("--cluster", "-c", default=None, help="Filter by cluster name")

    # delete
    p_del = subparsers.add_parser("delete", help="Delete a snapshot by name")
    p_del.add_argument("snapshot_name", help="VolumeSnapshot name to delete")
    p_del.add_argument("--namespace", "-n", default="topolvm-system", help="Namespace of the snapshot (default: topolvm-system)")

    # status
    p_status = subparsers.add_parser("status", help="Show detailed status of a snapshot")
    p_status.add_argument("snapshot_name", help="VolumeSnapshot name")
    p_status.add_argument("--namespace", "-n", default="topolvm-system", help="Namespace of the snapshot (default: topolvm-system)")

    # restore
    p_restore = subparsers.add_parser("restore", help="Restore a CNPG cluster from a snapshot")
    restore_sub = p_restore.add_subparsers(dest="restore_command", help="Restore sub-commands")

    # restore run
    p_restore_run = restore_sub.add_parser("run", help="Restore a cluster from a snapshot")
    p_restore_run.add_argument("cluster", help="Source CNPG cluster name (e.g. pgcluster-with-metrics)")
    p_restore_run.add_argument("--from-snapshot", "-s", required=True, help="VolumeSnapshot name to restore from")
    p_restore_run.add_argument("--new-cluster", "-n", default=None, help="Name for the new restored cluster (auto-generated if omitted)")
    p_restore_run.add_argument("--namespace", "-N", default="cnpg-system", help="Namespace of the CNPG cluster (default: cnpg-system)")
    p_restore_run.add_argument("--instances", "-i", type=int, default=1, help="Number of instances for restored cluster (default: 1)")
    p_restore_run.add_argument("--wait", "-w", action="store_true", help="Wait for restored cluster to be ready")
    p_restore_run.add_argument("--dry-run", "-d", action="store_true", help="Show what would be created without applying")

    # restore list-snapshots
    p_restore_list = restore_sub.add_parser("list-snapshots", help="List snapshots available for restore")
    p_restore_list.add_argument("--cluster", "-c", default=None, help="Filter by cluster name")
    p_restore_list.add_argument("--namespace", "-n", default="cnpg-system", help="Namespace to search (default: cnpg-system)")

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        sys.exit(1)

    dispatch = {
        "create": cmd_create,
        "list": list_snapshots,
        "delete": delete_snapshot,
        "status": snapshot_status,
    }

    if args.command == "restore":
        if not args.restore_command:
            p_restore.print_help()
            sys.exit(1)
        restore_dispatch = {
            "run": cmd_restore,
            "list-snapshots": list_snapshots_for_restore,
        }
        restore_dispatch[args.restore_command](args)
    else:
        dispatch[args.command](args)


if __name__ == "__main__":
    main()
