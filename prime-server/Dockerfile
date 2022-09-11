FROM alpine:3.14 as build
RUN apk add rustup gcc make
RUN rustup-init -y --default-toolchain stable
ADD ./ /server
RUN source /root/.profile && cd /server && cargo build --release

FROM alpine:3.14 as runtime
RUN apk add musl
COPY --from=build /server/target/release /release
ENTRYPOINT ["/release/prime-server"]
