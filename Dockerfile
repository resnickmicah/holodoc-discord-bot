FROM rust:1.78

WORKDIR /usr/src/holodoc
COPY . ./
RUN cargo build --release

ENTRYPOINT ["target/release/holodoc"]