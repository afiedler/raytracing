RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
	rustup run nightly \
	wasm-pack build --target web --release \
    ./ -Z build-std=panic_abort,std