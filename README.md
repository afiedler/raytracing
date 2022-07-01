# raytracing

![Raytracer Output](output.png?raw=true "Raytracer Output")

This is a Rust implementation of [Raytracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

There is both a web app and a CLI within this repo. Both are multithreaded, and the web app uses
[`SharedArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)
and [`wasm_thread`](https://crates.io/crates/wasm_thread) for threading.

## CLI

Run the CLI in release mode to make it much faster.

```
cargo run --release
```

## Web App

The web app is made of two parts: `raylib-web`, a lightweight wrapper library around the core raytracer in `raylib`,
and `raytracer-web` a React-based web app.

Build `raylib-web` with:

```
cd raylib-web
./build.sh
```

### Building `raytracer-web`

You need to serve the web app over HTTPS for `SharedArrayBuffer` to work. Create and install a certificate for
`localhost` with:

```
brew install mkcert
cd raytracer-web
mkdir -p .cert && mkcert -key-file ./.cert/key.pem -cert-file ./.cert/cert.pem 'localhost'
```

Next, start Vite:

```
# from within raytracer-web
npm install
npm run dev
```
