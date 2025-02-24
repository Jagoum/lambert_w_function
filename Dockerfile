# Build Stage
FROM rust:1.72 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Final Minimal Image
FROM debian:bookworm-slim  
# <- Updated from buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/lambert_w /usr/local/bin/lambert_w
ENTRYPOINT ["/usr/local/bin/lambert_w"]




# # syntax=docker/dockerfile:experimental
# FROM rust:latest as builder

# WORKDIR /app
# # Copy the source code into the container.
# COPY . .

# # Build the project in release mode.
# RUN rm -f Cargo.lock && cargo build --release 


# # Use a minimal base image to reduce final image size.
# FROM debian:buster-slim

# WORKDIR /app
# # Copy the binary from the builder stage.
# COPY --from=builder /app/target/release/lambert_w_function .

# # Set the startup command.
# CMD [ "./lambert_w_function" ]
