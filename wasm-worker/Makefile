
all:

clean:
	cargo clean

build:
	wasm-pack build --dev --no-pack --target web --out-dir ./dist/pkg --out-name wasm-worker

build-release:
	#wasm-pack build --release --no-pack --target web --out-dir ./dist/pkg --out-name wasm-worker
	wasm-pack build --release --target web --out-dir ./dist/pkg --out-name wasm-worker

