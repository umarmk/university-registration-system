# Use Debian-based Rust image for both build and runtime to avoid glibc issues
FROM rust:bullseye AS builder

# Set the working directory inside the container
WORKDIR /app

# Install necessary libraries, including PostgreSQL development files
RUN apt-get update && apt-get install -y libpq-dev

# Copy the entire project into the container
COPY . .

# Build the project in release mode
RUN cargo build --release

# Use the same Debian image for runtime
FROM rust:bullseye

# Install only the necessary runtime libraries for PostgreSQL
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Set up environment variables with defaults
ENV DATABASE_URL=postgres://postgres:postpotato1@db/university_registration \
    RUST_LOG=info \
    RUST_BACKTRACE=1

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/university_registration /usr/local/bin/university_registration

# Set proper permissions
RUN chown appuser:appuser /usr/local/bin/university_registration

# Switch to non-root user
USER appuser

# Expose the port your app runs on
EXPOSE 8081

# Add healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/health || exit 1

# Run the binary
CMD ["university_registration"]