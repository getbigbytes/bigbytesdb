FROM python:3.12-slim-bookworm

ARG TARGETPLATFORM
ENV TERM=dumb
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -y && \
    apt-get install -y apt-transport-https ca-certificates gdb curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /var/cache/apt/*
COPY ./distro/$TARGETPLATFORM/bigbytesdb-query /bigbytesdb-query
RUN useradd --uid 1000 --shell /sbin/nologin \
    --home-dir /var/lib/bigbytesdb --user-group \
    --comment "Bigbytesdb cloud data analytics" bigbytesdb && \
    mkdir -p /var/lib/bigbytesdb && \
    chown -R bigbytesdb:bigbytesdb /var/lib/bigbytesdb
ENTRYPOINT ["/bigbytesdb-query"]
