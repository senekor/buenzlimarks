# Deployment View

The web app is compiled and bundled using vite, the output goes into `app/dist`.
The backend server is run with `cargo run --release`.
It serves the compiled web app on top of its own REST api on port 4000.
On the server where the software is deployed, nginx or caddy may be used to handle SSL and forward the plain HTTP requests to the backend server.
Let's Encrypt and `certbot` may be used to acquire SSL certificates.

Dockerization is something to consider for a more streamlined deployment experience in the future.
