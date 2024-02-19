# SPDX-FileCopyrightText: 2024 Kamila Borowska <kamila@borowska.pw>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

FROM rust:1.76.0-slim-bookworm@sha256:de22cea71b620c7fdc61e8c1bf3f048d0ffbafe062ca9d7b32aed6a7d59109a4 AS base-rust-devel
RUN apt-get update \
 && apt-get install --no-install-recommends -y libpq-dev=15.5-0+deb12u1 libssl-dev=3.0.11-1~deb12u2 pkg-config=1.8.1-1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

FROM node:21.5.0-alpine3.19@sha256:9b54d010b382f0ef176dc93cd829bd4f2a905092b260746b3999aa824c9b7121 AS base-node-devel
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci

FROM base-node-devel AS vite
COPY vite.config.ts .
COPY js js
COPY static static
RUN npx vite build

FROM base-rust-devel AS builder
COPY build.rs Cargo.lock Cargo.toml ./
COPY src src
COPY --from=vite /app/dist/.vite/manifest.json dist/.vite/
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target cargo install --path .

FROM scratch AS combiner
WORKDIR /app
COPY migrations migrations
COPY static static
COPY templates templates
COPY languages.json .
COPY --from=builder /usr/local/cargo/bin/pastebinrun /usr/local/bin/
COPY --from=vite /app/dist dist

FROM base-rust-devel AS watcher
RUN cargo install cargo-watch
VOLUME /app
CMD ["cargo", "watch", "-x", "run", "-i", "js"]
ENV ROCKET_ADDRESS=0.0.0.0 ROCKET_DATABASES='{main={url="postgresql://postgres@postgres/"}}'
EXPOSE 8000/tcp

FROM base-node-devel AS watcher-vite
VOLUME /app
CMD ["npx", "vite", "--host"]
EXPOSE 5173/tcp

FROM debian:bookworm-20231218-slim@sha256:45287d89d96414e57c7705aa30cb8f9836ef30ae8897440dd8f06c4cff801eec
WORKDIR /app
RUN apt-get update \
 && apt-get install --no-install-recommends -y libpq5=15.5-0+deb12u1 libssl3=3.0.11-1~deb12u2 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*
COPY --from=combiner / /
CMD ["pastebinrun"]
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000/tcp
