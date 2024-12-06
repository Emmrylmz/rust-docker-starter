# Use the official Rust base image
FROM rust:alpine3.16

# Set the application directory
WORKDIR /app

# Install necessary dependencies
RUN apk add --no-cache musl-dev

# Update rustup and install the latest stable Rust and Cargo
RUN rustup self update && \
    rustup install stable && \
    rustup default stable

# Copy the project files to the container
COPY ./ ./

# Ensure dependencies are installed to avoid re-downloading during the build
RUN cargo fetch

# Install cargo-watch
RUN cargo install cargo-watch --locked

CMD ["cargo", "watch", "-x", "run"]