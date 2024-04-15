# Use the official Rust image as a parent image
FROM rust:1.77.2

# Copy the current directory contents into the container at /app
WORKDIR /app
COPY . .

# Build your program for release
RUN cargo build --release

EXPOSE 9876

# Run the binary
CMD ["./target/release/ferrodb"]
