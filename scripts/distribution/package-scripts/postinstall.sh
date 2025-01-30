#!/bin/sh
set -e

# Add Bigbytesdb to adm group to read /var/logs
usermod --append --groups adm bigbytesdb || true

if getent group 'systemd-journal'; then
	# Add Bigbytesdb to systemd-journal to read journald logs
	usermod --append --groups systemd-journal bigbytesdb || true
	systemctl daemon-reload || true
fi

if getent group 'systemd-journal-remote'; then
	# Add Bigbytesdb to systemd-journal-remote to read remote journald logs
	usermod --append --groups systemd-journal-remote bigbytesdb || true
	systemctl daemon-reload || true
fi
