# Build stage
FROM rust:1.73 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM ubuntu:22.04

# Install OpenSSL - often needed for Rust applications
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/app/target/release/weather-data-api .

# Expose the port the app runs on
EXPOSE 8080

# Command to run the application
CMD ["./weather-data-api"]