# Build release
 FROM rust:1-alpine3.24 as builder
 WORKDIR /app
 COPY . .
 RUN cargo build --release

# run the release file using the build file
FROM alpine:3.24
WORKDIR /app
COPY --from=builder /app/target/release/learn_rest .
EXPOSE 8000
CMD ./learn_rest
