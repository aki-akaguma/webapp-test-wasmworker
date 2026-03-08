// Import required functions.
import init, { initLog, runSingle, runPool } from "./wasm-worker.js";

async function init_wasm() {
    // Load wasm bindgen.
    await init();
    initLog();
}
await init_wasm();

async function kick_single(dioxus) {
    if (dioxus == null) { return; }
    let arg = await dioxus.recv();
    let ret = await runSingle(arg);
    dioxus.send(ret);
};

async function kick_worker(dioxus) {
    if (dioxus == null) { return; }
    let arg = await dioxus.recv();
    let ret = await runPool(arg);
    dioxus.send(ret);
};

function test(dioxus) {
    if (dioxus == null) { return; }
    return "Hello";
}

window.my = {
    test: test,
    kick_single: kick_single,
    kick_worker: kick_worker,
};
