use dioxus::prelude::*;

#[cfg(not(feature = "web"))]
use std::time::Instant;
#[cfg(feature = "web")]
use web_time::Instant;

const MAIN_JS: Asset = asset!("/assets/js/main.js", AssetOptions::js().with_minify(false));
//const MAIN_JS: Asset = asset!("/assets/js/main.js");

// This is necessary. Without it, `dx build --web --release` will not copy `worker.jp`.
const _PKG_WORKER_JS: Asset = asset!(
    "/assets/pkg/wasm-worker.js",
    AssetOptions::js()
        .with_minify(false)
        .with_hash_suffix(false)
);

#[component]
pub fn Home() -> Element {
    let mut single_elapsed = use_signal(|| 0.0);
    let mut multi_elapsed = use_signal(|| 0.0);
    let add_r = use_signal(|| 0);
    let fibo37_r = use_signal(|| 0);
    let fibo38_r = use_signal(|| 0);
    let fibo39_r = use_signal(|| 0);
    let fibo40_r = use_signal(|| 0);
    let fibo41_r = use_signal(|| 0);

    //    document::Script { src: MAIN_JS }
    rsx! {
        script { r#type: "module", src: MAIN_JS }
        div { " " }
        div {
            button {
                onclick: move |_evt| async move {
                    single_elapsed.set(0.0);
                    let start = Instant::now();
                    single_proc(add_r, fibo37_r, fibo38_r, fibo39_r, fibo40_r, fibo41_r).await;
                    let elapsed = (start.elapsed().as_millis() as f32) / 1000.0;
                    single_elapsed.set(elapsed);
                },
                "Single Worker"
            }
            span {
                " {single_elapsed} sec"
            }
        }
        br{}
        div {
            button {
                onclick: move |_evt| async move {
                    multi_elapsed.set(0.0);
                    let start = Instant::now();
                    multi_proc(add_r, fibo37_r, fibo38_r, fibo39_r, fibo40_r, fibo41_r).await;
                    let elapsed = (start.elapsed().as_millis() as f32) / 1000.0;
                    multi_elapsed.set(elapsed);
                },
                "Multi WORKER"
            }
            span {
                " {multi_elapsed} sec"
            }
        }
        br{}
        div {
            "2 + 3 => {add_r}"
        }
        br{}
        div { "fibonacci 37 => {fibo37_r}" }
        div { "fibonacci 38 => {fibo38_r}" }
        div { "fibonacci 39 => {fibo39_r}" }
        div { "fibonacci 40 => {fibo40_r}" }
        div { "fibonacci 41 => {fibo41_r}" }
    }
}

async fn single_proc(
    mut add_r: Signal<i32>,
    mut fibo37_r: Signal<i32>,
    mut fibo38_r: Signal<i32>,
    mut fibo39_r: Signal<i32>,
    mut fibo40_r: Signal<i32>,
    mut fibo41_r: Signal<i32>,
) {
    add_r.set(0);
    fibo37_r.set(0);
    fibo38_r.set(0);
    fibo39_r.set(0);
    fibo40_r.set(0);
    fibo41_r.set(0);
    //
    //
    let js = r#"{ await my.kick_single(dioxus); }"#;
    let mut eval = document::eval(js);
    //
    let arg = wasm_worker::RunArg {
        p1: vec![
            vec![1, 2, 3],
            vec![2, 41, 0],
            vec![2, 40, 0],
            vec![2, 39, 0],
            vec![2, 38, 0],
            vec![2, 37, 0],
        ],
    };
    eval.send(arg.to_json()).unwrap();
    //
    let r_json = eval.recv::<String>().await.unwrap();
    let r = wasm_worker::RunRet::from_json(&r_json);
    //
    for i in 0..r.p1.len() {
        let sw = r.p1[i][0];
        if sw == 1 {
            add_r.set(r.p1[i][3]);
        } else if sw == 2 {
            let a = r.p1[i][1];
            if a == 37 {
                fibo37_r.set(r.p1[i][3]);
            } else if a == 38 {
                fibo38_r.set(r.p1[i][3]);
            } else if a == 39 {
                fibo39_r.set(r.p1[i][3]);
            } else if a == 40 {
                fibo40_r.set(r.p1[i][3]);
            } else if a == 41 {
                fibo41_r.set(r.p1[i][3]);
            }
        }
    }
}

async fn multi_proc(
    mut add_r: Signal<i32>,
    mut fibo37_r: Signal<i32>,
    mut fibo38_r: Signal<i32>,
    mut fibo39_r: Signal<i32>,
    mut fibo40_r: Signal<i32>,
    mut fibo41_r: Signal<i32>,
) {
    add_r.set(0);
    fibo37_r.set(0);
    fibo38_r.set(0);
    fibo39_r.set(0);
    fibo40_r.set(0);
    fibo41_r.set(0);
    //
    //let js = r#"{ return my.test(dioxus); }"#;
    //let r = document::eval(js).await.unwrap();
    //dioxus_logger::tracing::debug!("PASS 000: {r}");
    //dioxus_logger::tracing::debug!("PASS 001");
    //
    let js = r#"{ await my.kick_worker(dioxus); }"#;
    let mut eval = document::eval(js);
    //
    let arg = wasm_worker::RunArg {
        p1: vec![
            vec![1, 2, 3],
            vec![2, 41, 0],
            vec![2, 40, 0],
            vec![2, 39, 0],
            vec![2, 38, 0],
            vec![2, 37, 0],
        ],
    };
    eval.send(arg.to_json()).unwrap();
    //
    let r_json = eval.recv::<String>().await.unwrap();
    let r = wasm_worker::RunRet::from_json(&r_json);
    //
    for i in 0..r.p1.len() {
        let sw = r.p1[i][0];
        if sw == 1 {
            add_r.set(r.p1[i][3]);
        } else if sw == 2 {
            let a = r.p1[i][1];
            if a == 37 {
                fibo37_r.set(r.p1[i][3]);
            } else if a == 38 {
                fibo38_r.set(r.p1[i][3]);
            } else if a == 39 {
                fibo39_r.set(r.p1[i][3]);
            } else if a == 40 {
                fibo40_r.set(r.p1[i][3]);
            } else if a == 41 {
                fibo41_r.set(r.p1[i][3]);
            }
        }
    }
}
