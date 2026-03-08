
wasm_worker_js_append = function dummy(arg) { if (arg == '') { return; } } dummy(wasm_bindgen);

MAIN_BASE = main
MAIN_ASSETS_PKG = $(MAIN_BASE)/assets/pkg
MAIN_ASSETS_PKG_WASM_WORKER_JS = $(MAIN_ASSETS_PKG)/wasm-worker.js
MAIN_PUBLIC_PKG = $(MAIN_BASE)/public/assets

WASM_WORKER_BASE = wasm-worker
WASM_WORKER_PKG = $(WASM_WORKER_BASE)/dist/pkg
WASM_WORKER_JS = $(WASM_WORKER_PKG)/wasm-worker.js
WASM_WORKER_WASM = $(WASM_WORKER_PKG)/wasm-worker_bg.wasm


all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

clean:
	cargo clean

check:
	cargo check

build-worker:
	make -C $(WASM_WORKER_BASE) build
	cp -a $(WASM_WORKER_JS) $(MAIN_ASSETS_PKG)/
	cp -a $(WASM_WORKER_WASM) $(MAIN_PUBLIC_PKG)/
	#echo "$(wasm_worker_js_append)" >> $(MAIN_ASSETS_PKG_WASM_WORKER_JS)

build-worker-release:
	make -C $(WASM_WORKER_BASE) build
	cp -a $(WASM_WORKER_JS) $(MAIN_ASSETS_PKG)/
	cp -a $(WASM_WORKER_WASM) $(MAIN_PUBLIC_PKG)/
	#echo "$(wasm_worker_js_append)" >> $(MAIN_ASSETS_PKG_WASM_WORKER_JS)

bundle-web: build-worker-release
	make -C $(MAIN_BASE) bundle-web

bundle-desktop: build-worker-release
	make -C $(MAIN_BASE) bundle-desktop

bundle-android-aarch64: build-worker-release
	make -C $(MAIN_BASE) bundle-android-aarch64

bundle-android-x86_64: build-worker-release
	make -C $(MAIN_BASE) bundle-android-x86_64

bundle-android-wva:
	make -C $(MAIN_BASE) bundle-android-wva

