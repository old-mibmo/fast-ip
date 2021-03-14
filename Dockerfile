ARG BUILD_IMAGE=registry.hub.docker.com/ekidd/rust-musl-builder:1.50.0
ARG RUNNER_IMAGE=scratch

FROM $BUILD_IMAGE as build

USER rust:rust

ADD --chown=rust:rust . ./
RUN cargo build --release

FROM $RUNNER_IMAGE

COPY --from=build \
    /home/rust/src/target/*/release/server \
    /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/server"  ]
