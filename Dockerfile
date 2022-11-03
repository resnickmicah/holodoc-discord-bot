FROM rust:1.64

WORKDIR /usr/src/holodoc
COPY . ./
RUN cargo build --release

ENTRYPOINT ["target/release/holodoc"]