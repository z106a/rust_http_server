# Rust Web Server

This project is a simple web server built using the Rust programming language. The server uses the popular Rocket web framework and serves different messages when accessed via a web browser or client through two routes: "/" and "/test".

## Getting Started

1. Make sure you have the latest version of Rust installed on your machine. You can download it from the official website at https://www.rust-lang.org/tools/install.

2. Clone the repository to your local machine using the following command:
```bash
git clone https://github.com/<username>/rust-web-server.git

Change into the project directory and build the project using the following command:

cd rust-web-server && cargo build
Run the server using the following command:
Copy code
cargo run
The server should now be running and accessible at http://localhost:8000.

Routes
http://localhost:8000/: serves a basic "Hello, World!" message
http://localhost:8000/test: serves a "This is a test route" message
