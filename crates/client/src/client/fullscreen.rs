use js_sys::Boolean;
use wasm_bindgen::prelude::*;

use crate::client::response::ClientResponse;

/// Request access to fullscreen and if allowed, provides fullscreen
/// functionality to a DOM element.
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)
#[wasm_bindgen(js_name = "requestFullscreen")]
pub async fn request_fullscreen() -> Result<ClientResponse, ClientResponse> {
    let window = web_sys::window()
        .expect("`window` doesn't exists. Make sure you are calling this function in the browser");
    let document = window
        .document()
        .expect("type `Document` doesn't exist in the current scope!");
    let document_element = document
        .document_element()
        .expect("error occured! expected document element");

    match document_element.request_fullscreen() {
        Ok(_) => {
            return Ok(ClientResponse::new("success", "Enabled full-screen"));
        }
        Err(e) => {
            return Err(ClientResponse::new(
                "error",
                &format!("Failed to enable full-screen: {:?}", e),
            ));
        }
    }
}

/// Exits fullscreen on the document. *Duh*
/// 
/// [MDN Documentation]()
#[wasm_bindgen(js_name = "exitFullscreen")]
pub async fn exit_fullscreen() {
    let window = web_sys::window()
        .expect("`window` doesn't exists. Make sure you are calling this function in the browser");
    let document = window
        .document()
        .expect("type `Document` doesn't exist in the current scope!");

    document.exit_fullscreen();
}

/// Check wether the document is currently in fullscreen mode or not.
/// Returns a boolean.
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreen)
#[wasm_bindgen(js_name = "isFullscreen")]
pub async fn is_fullscreen() -> Result<Boolean, JsValue> {
    let window = web_sys::window()
        .expect("`window` doesn't exists. Make sure you are calling this function in the browser");
    let document = window
        .document()
        .expect("type `Document` doesn't exist in the current scope!");

    if document.is_undefined() {
        return Err(JsValue::from_str("document is undefined"));
    } else {
        return Ok(js_sys::Boolean::from(document.fullscreen()));
    }
}

#[cfg(test)]
mod fullscreen_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_request_fullscreen() {
        let result = request_fullscreen().await;
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().get_status(), "success");
    }

    fn test_exit_fullscreen() {
        // What other test could fit in here?
        assert_eq!(5, 5);
        todo!("Fit assertions in here");
        // exit_fullscreen().await;
    }

    #[wasm_bindgen_test]
    async fn test_is_fullscreen() {
        let result = is_fullscreen().await;
        assert_eq!(result.is_ok(), true);
    }
}
