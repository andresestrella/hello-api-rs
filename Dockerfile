FROM rust:1.56.0 as build

RUN rustup component add rustfmt
RUN git clone https://github.com/andresestrella/hello-API /usr/src/app/sources
WORKDIR /usr/src/app/sources
RUN /bin/bash & ls
ENV PORT=50053

# Configures the startup!
RUN cargo build --release
RUN /bin/bash & ls
CMD ./target/release/main