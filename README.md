# 🎬 Axum Movie API (Single File)

A minimal REST API built with **Rust + Axum** — no database, just basic GET/POST functionality.

---

## 📦 Run the App

To run the app, execute the following command:

```bash
cargo run

```


The server will start at:
http://127.0.0.1:3000


## 🧪 API Endpoints

# GET /
Returns a welcome message.

# GET /movies/:id
Returns the movie ID from the path parameter.

# POST /movies
Accepts a JSON payload of a movie and echoes it back.

# Example
```bash

curl -X POST http://127.0.0.1:3000/movies \
  -H "Content-Type: application/json" \
  -d '{"id":1,"name":"Inception","year":2010,"was_good":true}'


```