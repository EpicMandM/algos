# First stage: Build the binary
FROM rust:1.63 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin algos
WORKDIR /algos

# Copy your source and manifest over
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build your application, release mode will optimize the binary size and performance
RUN cargo build --release

# Second stage: Setup the runtime environment
# Use a minimal Debian image to reduce the size
FROM debian:buster-slim

# Install necessary runtime dependencies, if any
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /algos/target/release/algos /usr/local/bin/algos

# Set the working directory to /data as this is where we'll mount our volume
WORKDIR /data

# Command to run the binary, expecting the data file to be in the current directory (/data)
CMD ["algos", "./10m.txt"]