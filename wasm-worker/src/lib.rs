use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

//use wasmworker::{iter_ext::IteratorExt, webworker, worker_pool, WebWorker};
//use wasmworker::{iter_ext::IteratorExt, webworker};
//use wasmworker::{webworker, worker_pool};
//use wasmworker::{webworker_channel, Channel};
//use wasmworker_proc_macro::webworker_channel_fn;
use wasmworker::webworker;
use wasmworker_proc_macro::webworker_fn;
//use wasmworker::*;
//use wasmworker_proc_macro::*;

#[wasm_bindgen(js_name = initLog)]
pub fn init_log() {
    //
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("initLog()");
    //
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = runSingle)]
pub async fn run_single(arg: String) -> String {
    // Run fibonacchi.
    //log::info!("on run_pool: {arg}");
    let mut a = RunArg::from_json(&arg);
    //log::info!("on run_pool: {a:?}");
    //
    let ret = a.p1.iter().map(|v| calc_proc(v)).collect::<Vec<i32>>();
    //
    for i in 0..ret.len() {
        a.p1[i].push(ret[i]);
    }
    //
    RunRet {
        ret: "Finish".to_string(),
        p1: a.p1,
    }
    .to_json()
}

#[wasm_bindgen(js_name = runPool)]
pub async fn run_pool(arg: String) -> String {
    use wasmworker::iter_ext::IteratorExt;
    // Run fibonacchi.
    //log::info!("on run_pool: {arg}");
    let mut a = RunArg::from_json(&arg);
    //log::info!("on run_pool: {a:?}");
    //
    let ret = a.p1.iter().par_map(webworker!(worker_proc)).await;
    //log::info!("par_map: {ret:?}");
    for i in 0..ret.len() {
        a.p1[i].push(ret[i]);
    }
    //
    RunRet {
        ret: "Finish".to_string(),
        p1: a.p1,
    }
    .to_json()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunArg {
    pub p1: Vec<Vec<i32>>,
}
impl RunArg {
    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunRet {
    pub ret: String,
    pub p1: Vec<Vec<i32>>,
}
impl RunRet {
    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[webworker_fn]
fn worker_proc(arg: Vec<i32>) -> i32 {
    calc_proc(&arg)
}

fn calc_proc(arg: &[i32]) -> i32 {
    if arg[0] == 1 {
        calc_add(arg[1], arg[2])
    } else if arg[0] == 2 {
        calc_fibonacci(arg[1])
    } else {
        0
    }
}

fn calc_add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn calc_fibonacci(num: i32) -> i32 {
    return match num {
        0 => 0,
        1 => 1,
        _ => calc_fibonacci(num - 1) + calc_fibonacci(num - 2),
    };
}
