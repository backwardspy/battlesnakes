ARG target=x86_64-unknown-linux-gnu

FROM rust:latest AS build

ARG target

WORKDIR /build
COPY Cargo.toml Cargo.toml
COPY src/ src/

ENV RUSTFLAGS "-C target-feature=+crt-static"
RUN cargo build --target $target --release --bin server

FROM scratch

ARG target

COPY --from=build /build/target/$target/release/server /server

EXPOSE 6502
CMD ["/server"]

