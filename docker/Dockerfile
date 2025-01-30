FROM debian:bookworm

ARG TARGETPLATFORM

RUN apt-get update -y && \
    apt-get install -y apt-transport-https ca-certificates curl xxd dumb-init && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/*

COPY ./distro/$TARGETPLATFORM/bigbytesdb-meta /usr/bin/bigbytesdb-meta
COPY ./distro/$TARGETPLATFORM/bigbytesdb-query /usr/bin/bigbytesdb-query
COPY ./docker/query-config.toml /etc/bigbytesdb/query_config_spec.toml
COPY ./docker/bootstrap.sh /bootstrap.sh
ENTRYPOINT [ "dumb-init", "--", "/bootstrap.sh"]
VOLUME [ "/var/log/bigbytesdb", "/etc/bigbytesdb", "/var/lib/bigbytesdb", "/var/lib/minio" ]
EXPOSE 3307 8124 8000 8900
