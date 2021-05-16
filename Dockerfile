FROM rust as builder
COPY . . 
RUN cargo build --release

FROM centos as runner
COPY --from=builder ./target/release/kube-event-exporter /kube-event-exporter
CMD ["/kube-event-exporter"]