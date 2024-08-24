# Single-Stage Build (Larger Image)
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install protoc (protobuf-compiler)
RUN apt-get update && apt-get install -y protobuf-compiler

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src
COPY proto ./proto
COPY build.rs ./
COPY words.txt ./
COPY words_alpha.txt ./

# Build the application with the specified binary target
RUN cargo build --release --bin helloworld-server

# Expose the port that the server will listen on (adjust as needed)
EXPOSE 8080

# Run the application
CMD ["./target/release/helloworld-server"]

