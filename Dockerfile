ARG RUST_VERS=1.44
ARG ALPINE_VERS=3.12

FROM rust:${RUST_VERS}-alpine${ALPINE_VERS} as proxy-builder
COPY . /src
WORKDIR /src
RUN apk add zeromq-dev musl-dev bash
ENV RUSTFLAGS="-C target-feature=-crt-static" 
RUN cargo build
RUN cargo test

CMD ["/bin/bash"]