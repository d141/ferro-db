# Use the official Rust image as a parent image
FROM rust:1.58

# Copy the current directory contents into the container at /app
WORKDIR /app
COPY . .

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/ferrodb"]
