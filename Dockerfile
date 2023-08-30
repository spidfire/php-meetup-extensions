# Use Ubuntu as the base image
FROM ubuntu:latest

# Set environment variables
ENV TZ=Europe/Amsterdam
ENV PATH="/root/.cargo/bin:${PATH}"

# Install essential packages and set time zone
RUN apt-get update &&  \
    ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone && \
    apt-get install -y curl build-essential php-cli php-dev make libclang-dev


# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
