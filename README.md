This README provides instructions on how to build and run a Rust-based application that performs various calculations on a dataset provided in the `10m.txt` file. You can build this application directly from the source code or use Docker to run the pre-built image or build your own Docker image.

## Prerequisites

- Rust installed on your machine (for building from source).
- Docker installed on your machine (for running or building a Docker image).

## 1. Build from Source

To build and run the application from source, follow these steps:

1. **Install Rust:**

   If you don't have Rust installed, you can install it using the following command:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   This command downloads and executes a script that installs the Rust toolchain. Follow the on-screen instructions to
   complete the installation.

2. **Build and Run the Application:**
   Navigate to the project directory and run the application using Cargo:

   ```sh
   cargo run
   ```

   Cargo will compile the project and execute it, expecting the `10m.txt` file to be present in the project's root directory.

## 2. Pull and Run the Docker Image

You can run the application using a pre-built Docker image without needing to install Rust.

1. **Prepare the Data File:**

   Place the `10m.txt` file in your working directory. This file will be mounted into the Docker container.

2. **Run the Docker Container:**

   Use the following command to run the application inside a Docker container:

   ```sh
   docker run -v ./:/data epicmandm/algo
   ```

   This command mounts the current directory (`./`) to `/data` inside the container, allowing the application to access the `10m.txt` file.

## 3. Build the Docker Image

If you prefer to build your own Docker image, follow these steps:

1. **Create a Dockerfile:**

   Ensure your project directory contains a Dockerfile with the necessary instructions to build the image (refer to the Dockerfile content provided in previous responses).

2. **Build the Docker Image:**

   In the project directory, run the following command to build the Docker image:

   ```sh
   docker build -t your-image-name .
   ```

   Replace `your-image-name` with the desired name for your Docker image.

3. **Run the Docker Container:**

   After building the image, run the application using the following command:

   ```sh
   docker run -v ./:/data your-image-name
   ```

   This command mounts the current directory (`./`) to `/data` in the container, allowing the application to access the `10m.txt` file when it runs.
