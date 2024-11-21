FROM alpine:3.20.3

WORKDIR /app

# Install base
RUN apk add curl \
    g++ gcc libc-dev \
    make openssl-dev \
    pkgconfig && \
    rm -rf /var/cache/apk/*

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

RUN cargo build --release

CMD ["./target/release/api"]