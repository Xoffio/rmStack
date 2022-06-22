use js_sys::{Function, Object, Reflect, WebAssembly};
//use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen(module = "/js/getStrings.js")]
extern "C" {
    fn just_hello(wasm: &JsValue) -> String;
    fn getString(wasm: &JsValue, nf_name: &str, name: &str) -> String;
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const WASM: &[u8] = include_bytes!("add_bg.wasm");

async fn run_async() -> Result<(), JsValue> {
    console_log!("instantiating a new wasm module directly");

    let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &Object::new())).await?;
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    //console_log!("# a: {:?}", a);
    //console_log!("# b: {:?}", b.to_string());

    let c = b.exports();

    //console_log!("# c: {:?}", c);

    let get_string_out = just_hello(c.as_ref());
    console_log!("# just_hello: {:?}", get_string_out);

    let get_string_out = getString(c.as_ref(), &"hello", "Potato");
    console_log!("# get_string_out: {:?}", get_string_out);

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run() {
    spawn_local(async {
        run_async().await.unwrap_throw();
    });
}
