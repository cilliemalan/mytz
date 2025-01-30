# syntax=docker/dockerfile:1

# docker build -t mytz .

ARG RUST_VERSION=1.81.0
ARG APP_NAME=mytz



FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git tzdata python3 openssl-dev openssl-libs-static



RUN --mount=type=bind,source=src,target=src,rw \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=gentz.py,target=gentz.py \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
python3 gentz.py && \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server




FROM alpine:3.18 AS final
# FROM scratch AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 3200

# What the container should run when it is started.
CMD ["/bin/server"]
