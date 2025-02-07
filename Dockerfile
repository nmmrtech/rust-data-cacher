# Use Rust official image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo files and source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the Rust application
RUN cargo build --release

# Expose the web server port
EXPOSE 8080

# Run the web server
CMD ["/app/target/release/rust-data-cacher"]
