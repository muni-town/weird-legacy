# Sycamore Spa Version

## Usage

3 steps: 
- Start the database
- Start the backend
- Serve the frontend

### Requirements
- Docker -- [installation](https://docs.docker.com/engine/install)
- Rust -- [installation](https://www.rust-lang.org/tools/install)
- Trunk -- [installation](https://trunkrs.dev/#install)

### Database
Edit the `ENV` variables in Dockerfile, and then build and run the docker image
```bash
docker build -t weird . && docker run --rm -p 5432:5432 weird

```

### Backend
Create a `.env` file at the root of the repository and add the `ENV` variables from the Dockerfile. 
Then start the server.
```bash
cargo run --bin backend
```
### Frontend
Serve the frontend.
```bash
cd frontend && trunk serve
```

Finally, visit `localhost:8080` in your browser
