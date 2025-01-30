#!/bin/sh
set -e

# Add bigbytes:bigbytes user & group
id --user bigbytes >/dev/null 2>&1 ||
	useradd --system --shell /sbin/nologin --home-dir /var/lib/bigbytes --user-group \
		--comment "Bigbytes cloud data analytics" bigbytes

# Create default Bigbytes data directory
mkdir -p /var/lib/bigbytes

# Make bigbytes:bigbytes the owner of the Bigbytes data directory
chown -R bigbytes:bigbytes /var/lib/bigbytes

# Create default Bigbytes log directory
mkdir -p /var/log/bigbytes

# Make bigbytes:bigbytes the owner of the Bigbytes log directory
chown -R bigbytes:bigbytes /var/log/bigbytes
