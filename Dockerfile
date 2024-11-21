# Use Debian-based Rust image for both build and runtime to avoid glibc issues
FROM rust:bullseye AS builder

# Set the working directory inside the container
WORKDIR /app

# Install necessary libraries, including PostgreSQL development files
RUN apt-get update && apt-get install -y libpq-dev

# Copy the entire project into the container
COPY . .

COPY .env .env

# Build the project in release mode
RUN cargo build --release

# Use the same Debian image for runtime
FROM rust:bullseye

# Install only the necessary runtime libraries for PostgreSQL
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Set up environment variables
ENV DATABASE_URL=postgres://postgres:postpotato1@db/university_registration

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/university_registration /usr/local/bin/university_registration

# Expose the port your app runs on
EXPOSE 8081

# Run the binary
CMD ["university_registration"]