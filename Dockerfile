FROM rust as builder
WORKDIR /app
COPY . . 
RUN cargo build --release

FROM debian:stable-slim as runner
COPY --from=builder /app/target/release/kube-event-exporter /kube-event-exporter
CMD ["/kube-event-exporter"]