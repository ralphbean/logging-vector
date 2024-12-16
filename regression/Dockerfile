#
# LADING
#
FROM ghcr.io/datadog/lading@sha256:fa37deaa777238953981d7628258d56d277e02a9b0dd93ca2d1688939236ce09 as lading

#
# VECTOR BUILDER
#
FROM ghcr.io/vectordotdev/vector/soak-builder@sha256:b598dc1eeb8661e4928238093a979597fd98f80554e5276d8c17cf4781ea3b82 as builder
WORKDIR /vector
COPY . .
RUN bash scripts/environment/install-protoc.sh
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/vector/target \
    /usr/local/bin/mold -run cargo build --bin vector --release && \
    cp target/release/vector .

#
# TARGET
#
FROM docker.io/debian:bookworm-slim@sha256:70d4c04302bdcd71c4fa21b6c12e99888380a07f04e3d44452b961bca046489d
RUN apt-get update && apt-get dist-upgrade -y && apt-get -y --no-install-recommends install zlib1g ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=lading /usr/bin/lading /usr/local/bin/lading
COPY --from=builder /vector/vector /usr/local/bin/vector
RUN mkdir --parents --mode=0777 /var/lib/vector

# Smoke test
RUN ["/usr/local/bin/lading", "--help"]
RUN ["/usr/local/bin/vector", "--version"]

ENTRYPOINT ["/usr/local/bin/lading"]
