# Use a Rust Docker image as the base
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/owner_login_ui

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove dummy source file
RUN rm -f src/main.rs

# Copy the source code
COPY src ./src

# Build the application
RUN cargo install --path .

# Create a new stage for the runtime image
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built executable from the previous stage
COPY --from=builder /usr/local/cargo/bin/owner_login_ui .

# Set the command to run the executable
CMD ["./myapp"]
