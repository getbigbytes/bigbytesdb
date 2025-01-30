#!/bin/sh
set -e

# Add Bigbytes to adm group to read /var/logs
usermod --append --groups adm bigbytes || true

if getent group 'systemd-journal'; then
	# Add Bigbytes to systemd-journal to read journald logs
	usermod --append --groups systemd-journal bigbytes || true
	systemctl daemon-reload || true
fi

if getent group 'systemd-journal-remote'; then
	# Add Bigbytes to systemd-journal-remote to read remote journald logs
	usermod --append --groups systemd-journal-remote bigbytes || true
	systemctl daemon-reload || true
fi
