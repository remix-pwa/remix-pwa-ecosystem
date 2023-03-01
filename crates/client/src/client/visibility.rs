use wasm_bindgen::prelude::*;
use web_sys::VisibilityState;

/// Returns the current visibility state of the document
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState)
#[wasm_bindgen(js_name = "getVisibilityState")]
pub fn get_visibility_state() -> Result<VisibilityState, JsValue> {
    let window = web_sys::window()
        .expect("`window` doesn't exists. Make sure you are calling this function in the browser");
    let document = window
        .document()
        .expect("type `Document` doesn't exist in the current scope!");

    Ok(document.visibility_state())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn document_visibility_test() {
        assert_eq!(get_visibility_state().unwrap(), VisibilityState::Visible);
    }
}
