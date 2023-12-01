#![allow(non_snake_case)]
use dioxus::prelude::*;

mod typescript {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(raw_module = "../node_modules/typescript/lib/typescript.js")]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["ts"], catch)]
        pub fn transpile(code: &str) -> Result<String, JsValue>;
    }
}

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let res = match typescript::transpile("let x = 5;") {
        Ok(res) => res,
        Err(err) => format!("Error: {:?}", err),
    };
    cx.render(rsx! {
        div {
            "{res}"
        }
    })
}
