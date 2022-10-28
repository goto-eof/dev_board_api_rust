FROM rust:latest
COPY ./ ./
RUN cargo build --release
EXPOSE 8000/tcp
CMD ["ENV=production ./target/release/dev-board"]