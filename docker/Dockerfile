FROM debian:bookworm

ARG TARGETPLATFORM

RUN apt-get update -y && \
    apt-get install -y apt-transport-https ca-certificates curl xxd dumb-init && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/*

COPY ./distro/$TARGETPLATFORM/bigbytes-meta /usr/bin/bigbytes-meta
COPY ./distro/$TARGETPLATFORM/bigbytes-query /usr/bin/bigbytes-query
COPY ./docker/query-config.toml /etc/bigbytes/query_config_spec.toml
COPY ./docker/bootstrap.sh /bootstrap.sh
ENTRYPOINT [ "dumb-init", "--", "/bootstrap.sh"]
VOLUME [ "/var/log/bigbytes", "/etc/bigbytes", "/var/lib/bigbytes", "/var/lib/minio" ]
EXPOSE 3307 8124 8000 8900
