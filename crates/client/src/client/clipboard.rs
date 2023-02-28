use log::{error, info};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use crate::client::response::ClientResponse;

// [short sentence explaining what it is]

// [more detailed explanation]

// [at least one code example that users can copy/paste to try it]

// [even more advanced explanations if necessary]

#[wasm_bindgen(js_name = "copyToClipboard")]
pub async fn copy_to_clipboard(text: String) -> Result<ClientResponse, ClientResponse> {
    let window = window().expect("Error occured! Are you calling this function in the server?");
    let navigator = window.navigator();

    if let Some(clipboard) = navigator.clipboard() {
        match JsFuture::from(clipboard.write_text(&text)).await {
            Ok(_) => Ok(ClientResponse::new("success", "Copied to clipboard!")),
            Err(_) => Err(ClientResponse::new(
                "error",
                "Error occured whilst copying to clipboard!",
            )),
        }
    } else {
        return Err(ClientResponse::new(
            "error",
            "Clipboard API doesn't seem available on your browser!",
        ));
    }
}

#[wasm_bindgen]
pub async fn paste_from_clipbaord() -> String {
    let window = window().expect("Error occured! Are you calling this function in the server?");
    let navigator = window.navigator();

    if let Some(clipboard) = navigator.clipboard() {
        match JsFuture::from(clipboard.read_text()).await {
            Ok(text) => {
                let text = text.as_string().unwrap();
                info!("Pasted text: {}", text);
                return text;
            }
            Err(_) => {
                error!("Error occured whilst pasting to clipboard!");
                return "Error occured whilst pasting to clipboard!".to_string();
            }
        }
    } else {
        let error_message = "Clipboard API doesn't seem available on your browser!";
        error!("{}", error_message);
        return error_message.to_string();
    }
}

/// *Unstable API* ☣️
#[wasm_bindgen]
pub async fn copy_image_to_clipboard(image: String) -> Result<ClientResponse, ClientResponse> {
    let window = window().expect("Error occured! Are you calling this function in the server?");
    let navigator = window.navigator();

    if let Some(clipboard) = navigator.clipboard() {
        let image = format!("data:image/png;base64,{}", image);

        match JsFuture::from(clipboard.write_text(&image)).await {
            Ok(_) => Ok(ClientResponse::new("success", "Copied to clipboard!")),
            Err(_) => Err(ClientResponse::new(
                "error",
                "Error occured whilst copying image to clipboard!",
            )),
        }
    } else {
        return Err(ClientResponse::new(
            "error",
            "Clipboard API doesn't seem available on your browser!",
        ));
    }
}

#[cfg(test)]
mod clipboard_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn copy_to_clipboard_test() {
        let text = "Hello World!";
        let result = copy_to_clipboard(text.to_string()).await;
        assert_eq!(result.is_ok(), true);
    }

    #[wasm_bindgen_test]
    async fn paste_from_clipboard_test() {
        let test_text = "Hello, Rust!";
        let text = String::from(test_text);

        copy_to_clipboard(text).await.unwrap();

        let result = paste_from_clipbaord().await;
        assert_eq!(result, test_text.to_string());
    }

    #[wasm_bindgen_test]
    async fn copy_image_to_clipboard_test() {
        let image = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAACWSURBVDhPY2AYBQAAADABARhI5DkAAAAASUVORK5CYII=";
        let result = copy_image_to_clipboard(image.to_string()).await;
        assert_eq!(result.is_ok(), true);
    }
}
