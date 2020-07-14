ARG RUST_VERS=1.44
ARG ALPINE_VERS=3.12

FROM rust:${RUST_VERS}-alpine${ALPINE_VERS} as proxy-builder
COPY . /src
WORKDIR /src
RUN apk add zeromq-dev musl-dev bash
RUN cargo build
# The produced binary tries to use runtime linker ld64.so instead of ld-musl-x86_64.
RUN ln -s /lib/ld-musl-x86_64.so.1 /lib/ld64.so.1
# fails, try running from bash
# RUN cargo test

CMD ["/bin/bash"]