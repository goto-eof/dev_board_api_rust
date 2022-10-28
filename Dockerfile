FROM rust:latest
COPY ./ ./
RUN cargo build --release
EXPOSE 8000/tcp
ENV DEV_BOARD_ENV production
CMD ["./target/release/dev-board"]