#!/bin/sh

set -e
CONFIG_DIR=/etc/transmission-daemon
SETTINGS="${CONFIG_DIR}/settings.json"
DEFAULTS=/etc/transmission-daemon-default.json

# A corrupted legacy state can leave a *directory* where settings.json
# belongs (which is also what breaks subPath file mounts at the runc
# level). The daemon cannot use it, so remove it and re-seed below.
# Guarded: if removal fails (e.g. NFS root-squash) the daemon will report
# the bad state instead of the container crash-looping on the script.
if [ -d "${SETTINGS}" ]; then
	rm -rf "${SETTINGS}" 2>/dev/null || true
	echo "Removed stale settings.json directory from ${CONFIG_DIR}"
fi

# A freshly provisioned volume has no settings file; seed it with the image
# defaults so the daemon always starts with valid JSON.
if [ ! -f "${SETTINGS}" ] && [ -f "${DEFAULTS}" ]; then
	if cp "${DEFAULTS}" "${SETTINGS}"; then
		echo "Seeded ${SETTINGS} from defaults"
	fi
fi

# First-time bootstrap: fill in the RPC credentials from the environment.
# The replacement goes through a temp file + rename: rename only needs write
# access to the config directory (fsGroup provides it), not to the existing
# file, so this works even if a root-owned settings.json is still on the
# volume. Every step is guarded: a failure must never crash-loop the
# container, it just means the credentials were already bootstrapped.
if [ -f "${SETTINGS}" ] && grep -q "#rpc-password#" "${SETTINGS}"; then
	if sed "s/#rpc-password#/$PASSWORD/g; s/#rpc-username#/$USERNAME/g" "${SETTINGS}" \
		> "${SETTINGS}.new" \
		&& mv "${SETTINGS}.new" "${SETTINGS}"; then
		echo "Bootstrapped RPC credentials into ${SETTINGS}"
	else
		echo >&2 "WARNING: could not bootstrap ${SETTINGS}; starting with existing settings"
		rm -f "${SETTINGS}.new"
	fi
fi

unset PASSWORD USERNAME

exec /usr/bin/transmission-daemon --foreground --config-dir "${CONFIG_DIR}"
