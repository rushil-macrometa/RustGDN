# Use an official Rust image as a parent image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to cache dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Build the dependencies to cache them
RUN cargo build --release
RUN rm -f target/release/deps/myapp*

# Now copy the rest of the source code
COPY . .

# Build the actual application
RUN cargo build --release

# Set the binary name and add it to the PATH
ENV PATH="/usr/src/myapp/target/release:${PATH}"

# Run the binary
CMD ["./target/release/myapp"]
