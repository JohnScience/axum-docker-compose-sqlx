# Use a base image with the latest version of Rust installed
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Install C/C++ musl toolchain (a lot of crates may need "clang" as well)
RUN apt-get update && apt-get install -y musl-tools

# Install the linux-musl build target
RUN rustup target add x86_64-unknown-linux-musl

# Create a blank project
RUN cargo init

# Copy only the dependencies
COPY Cargo.toml Cargo.lock ./

# A dummy build to get the dependencies compiled and cached
RUN cargo build --target x86_64-unknown-linux-musl --release

# Copy the real application code into the container
COPY src src

# Build the application
RUN cargo build --target x86_64-unknown-linux-musl --release

# (Optional) Remove debug symbols
RUN strip target/x86_64-unknown-linux-musl/release/adcs-example

# Use a slim image for running the application
FROM alpine as runtime

# Copy only the compiled binary from the builder stage to this image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/adcs-example /bin/adcs-example

# Specify the command to run when the container starts
CMD ["/bin/adcs-example"]
