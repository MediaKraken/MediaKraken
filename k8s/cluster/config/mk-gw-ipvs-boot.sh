#!/usr/bin/env bash
# Restore the gateway VIP IPVS Route-mode entries on this node.
#
# Background (verified 2026-11-05):
#  - This cluster runs Cilium with kube-proxy-replacement=true; there is no
#    kube-proxy, so after a node reboot NOTHING rebuilds the kernel IPVS
#    table (it is kernel memory only and wiped on every boot).
#  - The gateway service mediakraken/cilium-gateway-mkstack-webapp has
#    selector <none> and no endpoints. Verified 2026-11-05 on
#    192.168.50.50 with all four gateway entries removed: Cilium's eBPF
#    (bpf_sock sk_lookup -> per-node envoy) carries the VIP traffic
#    (192.168.50.202:443/80 and the per-node NodePorts 31528/32289) on its
#    own when the VSes are simply ABSENT. The broken state is a VS that
#    EXISTS with zero real servers (IPVS intercepts the packet and answers
#    ICMP port-unreachable — the 2026-11-05 cutover failure mode). This
#    script restores the exact production configuration — the four
#    Route-mode real servers:
#        <VIP>:<port>  ->  127.0.0.1:<port>   (Route mode, NO -m/Masq)
#    IPVS rewrites the destination to 127.0.0.1:<port>; Cilium's bpf_sock
#    sk_lookup then redirects the connection to the per-node envoy L7 proxy
#    listener (whose port is NOT stable across envoy restarts and must
#    never be hardcoded here).
#  - Masq (-m) to a loopback real server is BROKEN in this stack (silent
#    timeout), so the plain add without -m is mandatory; the kernel
#    normalizes it to "Route".
#
# This script is idempotent and safe to run at any time, not only at boot.
# On failure it exits non-zero: mk-gateway-ipvs.service then enters the
# "failed" state and shows up in systemctl status.
#
# Keep the VIP list below in sync with playbooks/cilium_gateway.yml
# (play "Maintain gateway VIP IPVS entries (Route mode)").
set -euo pipefail

GW_VIP=192.168.50.202
GW_VIP_PORT_443=443
GW_VIP_PORT_80=80
NODE_PORT_443=31528
NODE_PORT_80=32289

# 1. The ip_vs kernel module (its table is empty right after a boot).
modprobe ip_vs

# 2. Wait until ipvsadm can talk to the module.
for _ in $(seq 1 30); do
  ipvsadm -Ln >/dev/null 2>&1 && break
  sleep 1
done
if ! ipvsadm -Ln >/dev/null 2>&1; then
  echo "mk-gw-ipvs-boot: ERROR: ipvsadm cannot reach ip_vs after 30s" >&2
  exit 1
fi

# 3. Wait for this node's 192.168.50.x management IP (up to 120s).
NODE_IP=""
for _ in $(seq 1 120); do
  NODE_IP=$(ip -4 -o addr show 2>/dev/null | awk '{print $4}' | cut -d/ -f1 \
    | grep -E '^192\.168\.50\.[0-9]+$' | head -1 || true)
  [ -n "$NODE_IP" ] && break
  sleep 1
done
if [ -z "$NODE_IP" ]; then
  echo "mk-gw-ipvs-boot: ERROR: no 192.168.50.x interface IP after 120s" >&2
  exit 1
fi

VP_LIST="$GW_VIP:$GW_VIP_PORT_443
$GW_VIP:$GW_VIP_PORT_80
$NODE_IP:$NODE_PORT_443
$NODE_IP:$NODE_PORT_80"

for vp in $VP_LIST; do
  port=${vp##*:}
  # Create the virtual service if missing. After a boot nothing else creates
  # these VSes (kube-proxy is gone), and `ipvsadm -a` does NOT auto-create a
  # missing service (verified 2026-11-05), so this step is required.
  # -A on an existing service fails with "already defined"; that is fine.
  ipvsadm -A -t "$vp" -s rr 2>/dev/null || true
  # Drop every existing real server for this VIP, then add the plain
  # (no -m) loopback real server; the kernel stores it as Route mode.
  while :; do
    rs=$(ipvsadm -Ln 2>/dev/null \
      | awk -v a="$vp" '$1=="TCP" {show=($2==a); next} show && /^  -> / {print $2; exit}')
    [ -n "$rs" ] || break
    ipvsadm -d -t "$vp" -r "$rs" 2>/dev/null || break
  done
  ipvsadm -a -t "$vp" -r "127.0.0.1:$port"
done

# 4. Verify: every VIP must have a Route-mode 127.0.0.1 real server.
rc=0
for vp in $VP_LIST; do
  if ! ipvsadm -Ln 2>/dev/null \
    | awk -v a="$vp" '$1=="TCP" {show=($2==a); next} show && /^  -> / {print $2, $3}' \
    | grep -Eq '127\.0\.0\.1:[0-9]+ +Route'; then
    echo "mk-gw-ipvs-boot: ERROR: gateway VIP $vp has no Route-mode 127.0.0.1 real server" >&2
    rc=1
  fi
done
if [ "$rc" -ne 0 ]; then
  exit 1
fi
echo "mk-gw-ipvs-boot: OK (node $NODE_IP, 4 Route-mode entries in place)"
