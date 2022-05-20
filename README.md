# raytracing

![Raytracer Output](output.png?raw=true "Raytracer Output")

This is a Rust implementation of [Raytracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

There is both a web app and a CLI within this repo. Currently, the CLI is multithreaded, but the web version is not.

## CLI

Run the CLI in release mode to make it much faster.

```
cargo run --release
```

## Web App

The web app is very slow and raytraces on the main thread. I hope to move it into a web worker later. Start the web app with:

```
cargo xtask start
```

Build an optimized version with and serve with [http-server](https://crates.io/crates/http-server)

```
cargo xtask dist --release
http-server target/release/dist --port 8000
```
