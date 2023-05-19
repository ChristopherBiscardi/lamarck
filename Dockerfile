# Use a base image that matches the target platform
FROM ubuntu:latest

# Install any necessary dependencies or libraries required by your program
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libcurl4-openssl-dev \
    pkg-config \
    libpq-dev
    
# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary into the image
COPY target/release/ .
RUN mkdir audio
COPY audio/** audio/

# Expose any necessary ports for your program (if applicable)
EXPOSE 8080

# Set the entry point or command to run your program when the container starts
CMD ["./lamarck"]
