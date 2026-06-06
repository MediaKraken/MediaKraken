#!/usr/bin/env python3
"""
Manage CloudNativePG Barman object-store backups (create, list, restore).

Usage:
    cnpg_backup.py create <cluster> [--namespace cnpg-system] [--wait]
    cnpg_backup.py list     [--namespace cnpg-system] [--cluster <cluster>]
    cnpg_backup.py restore  <cluster> --from-backup <backup-name> [--new-cluster <name>] [--namespace cnpg-system] [--wait]

Requires: kubectl (with access to the cluster), Python 3.8+
"""

import argparse
import json
import subprocess
import sys
from datetime import datetime, timezone
from typing import Optional

BACKUP_NAMESPACE = "cnpg-system"
CLUSTER_NAMESPACE = "cnpg-system"


def run_cmd(args: list[str], check: bool = True) -> subprocess.CompletedProcess:
    """Run a command and return the result."""
    try:
        return subprocess.run(args, capture_output=True, text=True, check=check)
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr.strip()}", file=sys.stderr)
        sys.exit(1)


def get_cluster_spec(cluster: str, namespace: str) -> Optional[dict]:
    """Fetch the original cluster spec as a template."""
    cmd = [
        "kubectl", "get", "cluster", cluster,
        "-n", namespace,
        "-o", "json",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        return None
    return json.loads(result.stdout)


def create_backup(cluster: str, namespace: str, wait: bool = False) -> str:
    """Create a CNPG Backup CR for the given cluster."""
    backup_name = f"backup-{cluster}-{generate_timestamp()}"

    backup_yaml = {
        "apiVersion": "postgresql.cnpg.io/v1",
        "kind": "Backup",
        "metadata": {
            "name": backup_name,
            "namespace": namespace,
            "labels": {
                "created-by": "cnpg_backup.py",
                "created-at": datetime.now(timezone.utc).isoformat(),
            },
        },
        "spec": {
            "cluster": {
                "name": cluster,
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

    if wait:
        print(f"Waiting for Backup '{backup_name}' to complete...")
        if not wait_for_backup(backup_name, namespace):
            print(f"Backup '{backup_name}' did not complete in time.", file=sys.stderr)
            sys.exit(2)

    return backup_name


def list_backups(namespace: str, cluster: Optional[str] = None):
    """List Backup CRs, optionally filtered by cluster."""
    cmd = [
        "kubectl", "get", "backup",
        "-n", namespace,
        "-o", "custom-columns="
              "NAME:.metadata.name,"
              "CLUSTER:.spec.cluster.name,"
              "PHASE:.status.phase,"
              "START:.status.startBackupEnd,"
              "END:.status.endBackupEnd,"
              "SIZE:.status.backupStartLSN,"
              "AGE:.metadata.creationTimestamp",
    ]

    if cluster:
        cmd += ["-l", f"cnpg.io/cluster={cluster}"]

    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        print(f"Error listing backups: {result.stderr.strip()}", file=sys.stderr)
        return

    output = result.stdout.strip()
    if not output or "No resources found" in output:
        print("No backups found.")
        return
    print(output)


def list_backups_detailed(namespace: str, cluster: Optional[str] = None):
    """List backups with detailed information including error messages and S3 paths."""
    cmd = [
        "kubectl", "get", "backup",
        "-n", namespace,
        "-o", "json",
    ]

    if cluster:
        cmd += ["-l", f"cnpg.io/cluster={cluster}"]

    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        print(f"Error listing backups: {result.stderr.strip()}", file=sys.stderr)
        return

    data = json.loads(result.stdout)
    items = data.get("items", [])

    if not items:
        print("No backups found.")
        return

    print(f"\n{'NAME':<45} {'CLUSTER':<35} {'PHASE':<12} {'AGE':<20} {'ERROR'}")
    print("-" * 140)

    for item in items:
        meta = item.get("metadata", {})
        status = item.get("status", {})
        spec = item.get("spec", {})

        name = meta.get("name", "N/A")
        bk_cluster = spec.get("cluster", {}).get("name", "N/A")
        phase = status.get("phase", "unknown")
        created = meta.get("creationTimestamp", "N/A")
        error = status.get("error", "") or ""

        # Calculate age
        try:
            created_dt = datetime.fromisoformat(created.replace("Z", "+00:00"))
            age = datetime.now(timezone.utc) - created_dt
            days = age.days
            hours, remainder = divmod(age.seconds, 3600)
            age_str = f"{days}d {hours}h" if days else f"{hours}h"
        except (ValueError, TypeError):
            age_str = created

        error_str = error if len(error) < 40 else error[:37] + "..."
        print(f"{name:<45} {bk_cluster:<35} {phase:<12} {age_str:<20} {error_str}")

    print()


def wait_for_backup(backup_name: str, namespace: str, timeout: int = 3600) -> bool:
    """Wait for a Backup CR to reach 'ready' phase."""
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

        data = json.loads(result.stdout)
        phase = data.get("status", {}).get("phase", "unknown")
        print(f"  Phase: {phase}")

        if phase in ["ready", "completed"]:
            print(f"Backup '{backup_name}' is ready.")
            return True
        elif phase in ["failed", "error"]:
            error = data.get("status", {}).get("error", "unknown error")
            print(f"Backup '{backup_name}' failed: {error}", file=sys.stderr)
            return False

        elapsed = (datetime.now(timezone.utc) - start).total_seconds()
        if elapsed >= timeout:
            print(f"Timeout after {timeout}s.", file=sys.stderr)
            return False

        import time
        time.sleep(15)


def generate_timestamp() -> str:
    """Generate a timestamp string suitable for naming."""
    return datetime.now(timezone.utc).strftime("%Y%m%d%H%M%S")


def restore_from_backup(
    cluster: str,
    backup_name: str,
    namespace: str,
    new_cluster_name: Optional[str] = None,
    wait: bool = False,
    instances: int = 1,
    dry_run: bool = False,
):
    """Restore a CNPG cluster from a Barman backup."""
    # Verify backup exists and get its details
    cmd = [
        "kubectl", "get", "backup", backup_name,
        "-n", namespace,
        "-o", "json",
    ]
    result = run_cmd(cmd, check=False)
    if result.returncode != 0:
        print(f"Error: Backup '{backup_name}' not found in namespace '{namespace}'", file=sys.stderr)
        sys.exit(1)

    backup_data = json.loads(result.stdout)
    backup_phase = backup_data.get("status", {}).get("phase", "")
    backup_cluster = backup_data.get("spec", {}).get("cluster", {}).get("name", cluster)

    if backup_phase not in ["ready", "completed"]:
        print(f"WARNING: Backup '{backup_name}' phase is '{backup_phase}', not 'ready'. Restore may fail.", file=sys.stderr)

    print(f"Backup:     {backup_name}")
    print(f"Source:     {backup_cluster}")
    print(f"Phase:      {backup_phase}")

    # Get original cluster spec
    print(f"Loading cluster spec from '{backup_cluster}'...")
    original_spec = get_cluster_spec(backup_cluster, namespace)
    if not original_spec:
        print(f"Error: Could not find cluster '{backup_cluster}' in namespace '{namespace}'", file=sys.stderr)
        sys.exit(1)

    original_spec = original_spec.get("spec", {})

    # Determine new cluster name
    if new_cluster_name is None:
        base = backup_name.replace("backup-", "")
        new_cluster_name = f"{base}-restore"

    # Dry run
    if dry_run:
        print(f"\n=== DRY RUN: Would create cluster '{new_cluster_name}' ===\n")
        cluster_yaml = {
            "apiVersion": "postgresql.cnpg.io/v1",
            "kind": "Cluster",
            "metadata": {
                "name": new_cluster_name,
                "namespace": namespace,
                "labels": {
                    "restored-from": backup_name,
                    "restored-by": "cnpg_backup.py",
                },
            },
            "spec": {
                "instances": instances,
                "imageName": original_spec.get("imageName", "N/A"),
                "primaryUpdateStrategy": original_spec.get("primaryUpdateStrategy", "unsupervised"),
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
                "pgbouncer": original_spec.get("pgbouncer"),
            },
        }
        print(json.dumps(cluster_yaml, indent=2))
        return

    # Create new cluster from backup
    cluster_yaml = {
        "apiVersion": "postgresql.cnpg.io/v1",
        "kind": "Cluster",
        "metadata": {
            "name": new_cluster_name,
            "namespace": namespace,
            "labels": {
                "restored-from": backup_name,
                "restored-by": "cnpg_backup.py",
            },
        },
        "spec": {
            "instances": instances,
            "imageName": original_spec.get("imageName", "N/A"),
            "primaryUpdateStrategy": original_spec.get("primaryUpdateStrategy", "unsupervised"),
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

    if wait:
        print(f"\nWaiting for cluster '{new_cluster_name}' to become ready...")
        if not wait_for_cluster_ready(new_cluster_name, namespace):
            print(f"Cluster '{new_cluster_name}' may not be fully ready. Check with:", file=sys.stderr)
            print(f"  kubectl get cluster {new_cluster_name} -n {namespace}", file=sys.stderr)
            sys.exit(2)

    print(f"\nRestore complete!")
    print(f"  Restored Cluster: {new_cluster_name}")
    print(f"  From Backup:      {backup_name}")
    print(f"\nTo connect to the restored cluster:")
    print(f"  kubectl get secret {new_cluster_name}-app-user -n {namespace} -o jsonpath='{{.data.username}}' | base64 -d")
    print(f"  kubectl get secret {new_cluster_name}-app-user -n {namespace} -o jsonpath='{{.data.password}}' | base64 -d")
    print(f"  Host: {new_cluster_name}-pgbouncer-rw.{namespace}.svc.cluster.local")


def wait_for_cluster_ready(cluster_name: str, namespace: str, timeout: int = 900) -> bool:
    """Wait for a CNPG cluster to become ready."""
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


def cmd_create(args):
    backup_name = create_backup(
        args.cluster,
        args.namespace,
        args.wait,
    )
    print(f"\nBackup created: {backup_name}")
    print(f"Status: kubectl get backup {backup_name} -n {args.namespace}")


def cmd_list(args):
    if args.detailed:
        list_backups_detailed(args.namespace, args.cluster)
    else:
        list_backups(args.namespace, args.cluster)


def main():
    parser = argparse.ArgumentParser(
        description="Manage CloudNativePG Barman object-store backups",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # create
    p_create = subparsers.add_parser("create", help="Create a new backup for a CNPG cluster")
    p_create.add_argument("cluster", help="CNPG cluster name (e.g. pgcluster-with-metrics)")
    p_create.add_argument("--namespace", "-n", default=CLUSTER_NAMESPACE, help=f"Namespace (default: {CLUSTER_NAMESPACE})")
    p_create.add_argument("--wait", "-w", action="store_true", help="Wait for backup to complete")

    # list
    p_list = subparsers.add_parser("list", help="List backups")
    p_list.add_argument("--namespace", "-n", default=BACKUP_NAMESPACE, help=f"Namespace (default: {BACKUP_NAMESPACE})")
    p_list.add_argument("--cluster", "-c", default=None, help="Filter by cluster name")
    p_list.add_argument("--detailed", "-D", action="store_true", help="Show detailed backup info")

    # restore
    p_restore = subparsers.add_parser("restore", help="Restore a cluster from a backup")
    p_restore.add_argument("cluster", help="Source CNPG cluster name (for spec lookup)")
    p_restore.add_argument("--from-backup", "-b", required=True, help="Backup name to restore from")
    p_restore.add_argument("--new-cluster", "-n", default=None, help="Name for the restored cluster (auto-generated if omitted)")
    p_restore.add_argument("--namespace", "-N", default=BACKUP_NAMESPACE, help=f"Namespace (default: {BACKUP_NAMESPACE})")
    p_restore.add_argument("--instances", "-i", type=int, default=1, help="Number of instances (default: 1)")
    p_restore.add_argument("--wait", "-w", action="store_true", help="Wait for restored cluster to be ready")
    p_restore.add_argument("--dry-run", "-d", action="store_true", help="Show what would be created without applying")

    args = parser.parse_args()
    if not args.command:
        parser.print_help()
        sys.exit(1)

    dispatch = {
        "create": cmd_create,
        "list": cmd_list,
        "restore": restore_from_backup,
    }

    dispatch[args.command](args)


if __name__ == "__main__":
    main()
