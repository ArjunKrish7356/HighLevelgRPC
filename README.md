# gRPC Communication using Rust

This project demonstrates a simple yet powerful gRPC-based sorting service. By leveraging the efficiency of gRPC for remote procedure calls, this service allows you to send an array of numbers to the server, which will then process the request and return the sorted array back to the client.
## Setup Instructions

1. Install Protocol Buffers Compiler:
   ```
   sudo apt install protobuf-compiler
   ```

2. Build the Project:
   ```
   cargo build
   ```

3. Run the Server:
   ```
   cargo run --bin server
   ```

4. Run the Client (in a separate terminal):
   ```
   cargo run --bin client
   ```
