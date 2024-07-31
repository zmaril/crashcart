# Use a base image with the required tools
FROM ubuntu:latest

# Install dependencies
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    gcc \
    libbpf-dev \
    libelf-dev \
    make \
    linux-headers-$(uname -r) \
    git \
    build-essential \
    pkg-config \
    iproute2

RUN ln -s /usr/include/aarch64-linux-gnu/asm /usr/include/asm

# Copy the source files
COPY hello_bpf.c /usr/src/hello_bpf.c
COPY main.c /usr/src/main.c

# Set the working directory
WORKDIR /usr/src

# Compile the BPF program
RUN clang -target bpf -c hello_bpf.c -o hello_bpf.o

# Compile the user-space program
RUN clang -o main main.c -lbpf

# Command to run the user-space program
CMD ["./main"]
