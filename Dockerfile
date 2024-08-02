FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04 as Final
COPY --from=builder /app/target/release/todo-list_BE /
EXPOSE 3005
CMD ["./todo-list_BE"]

