#!/bin/sh
set -e

# Add bigbytesdb:bigbytesdb user & group
id --user bigbytesdb >/dev/null 2>&1 ||
	useradd --system --shell /sbin/nologin --home-dir /var/lib/bigbytesdb --user-group \
		--comment "Bigbytesdb cloud data analytics" bigbytesdb

# Create default Bigbytesdb data directory
mkdir -p /var/lib/bigbytesdb

# Make bigbytesdb:bigbytesdb the owner of the Bigbytesdb data directory
chown -R bigbytesdb:bigbytesdb /var/lib/bigbytesdb

# Create default Bigbytesdb log directory
mkdir -p /var/log/bigbytesdb

# Make bigbytesdb:bigbytesdb the owner of the Bigbytesdb log directory
chown -R bigbytesdb:bigbytesdb /var/log/bigbytesdb
