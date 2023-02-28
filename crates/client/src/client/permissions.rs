use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn get_permission_status(permission: String) -> Result<JsValue, JsValue> {
    let window = web_sys::window().expect("no global `window` exists. Make sure you are calling this function in the browser");
    let navigator = window.navigator();

    let permissions = navigator.permissions().expect("no permissions object found");

    let permission_object = Object::new();
    Reflect::set(&permission_object, &"name".into(), &permission.as_str().into()).unwrap();
    
    let permission_status = JsFuture::from(permissions.query(&permission_object).expect("no permission status found")).await;

    if permission_status.is_ok() {
      Ok(permission_status.unwrap())
    } else {
      Err(permission_status.err().unwrap())
    }
}

#[cfg(test)]
mod permissions_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn get_permissions_test() {
        let permission_status = get_permission_status("clipboard-read".to_string()).await;
        assert!(permission_status.is_ok());
    }
}