FROM rust:latest AS build

# View app name in Cargo.toml
ARG APP_NAME=api

WORKDIR /build

COPY . .

RUN cargo build --locked --release

RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final

COPY --from=build /bin/server /bin/

CMD ["/bin/server"]