FROM debian:bookworm

ARG TARGETPLATFORM
ENV TERM=dumb
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y && \
    apt-get install -y apt-transport-https ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/*
COPY ./distro/$TARGETPLATFORM/bigbytesdb-meta /bigbytesdb-meta
COPY ./distro/$TARGETPLATFORM/bigbytesdb-metactl /bigbytesdb-metactl
RUN useradd --uid 1000 --shell /sbin/nologin \
    --home-dir /var/lib/bigbytesdb --user-group \
    --comment "Bigbytesdb cloud data analytics" bigbytesdb && \
    mkdir -p /var/lib/bigbytesdb && \
    chown -R bigbytesdb:bigbytesdb /var/lib/bigbytesdb
ENTRYPOINT ["/bigbytesdb-meta"]
