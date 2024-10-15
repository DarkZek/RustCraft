FROM rustlang/rust:nightly

ARG API_URL
ARG PUBLIC_JWT_KEY
ARG SSL_CERTIFICATE
ARG SSL_PRIVATE_KEY
ARG ASSETS_DIR

RUN apt update
RUN apt install -y libwayland-dev libasound2-dev libudev-dev

COPY --from=repo . /build/

WORKDIR /build/

RUN cargo build --release --bin rc_server
RUN cp ./target/release/rc_server /etc/rc_server
RUN cp -R ./assets /assets

RUN rm -Rf /build

WORKDIR /server/

CMD ["/etc/rc_server"]