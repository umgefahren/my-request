FROM rust:1.62 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release


FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/my-request /usr/local/bin/my-request
CMD ["my-request"]
EXPOSE 8080