FROM rust:latest
COPY ./ ./
RUN cargo build --release
EXPOSE 8000/tcp
CMD ["./target/release/dev-board"]