FROM rust:alpine3.20 as builder

# Install necessary build dependencies
RUN apk add --no-cache musl-dev gcc openssl-dev

WORKDIR /app

# Copy the Cargo.toml file to build the dependency cache
COPY Cargo.toml ./

# Now copy the actual source code
COPY ./src ./src

# Build the Rust application in release mode
RUN cargo build --release

# Stage 2: Create a minimal image
FROM alpine:3.20

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/file_organizer_cli_tool_rust /usr/local/bin/file_organizer

# Expose the desired port
EXPOSE 8082

# Set the entry point for the container
ENTRYPOINT ["/usr/local/bin/file_organizer"]