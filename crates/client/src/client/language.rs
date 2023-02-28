use log::{error, info};
use wasm_bindgen::prelude::*;
use web_sys::window;


/// Gets the language of the user's browser.
/// 
/// ## Example
/// 
/// ```tsx
/// const preferredLanguage = await getLanguage();
/// setLanguageContext(preferredLanguage); 
/// ```
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)
#[wasm_bindgen(js_name = "getLanguage")]
pub async fn get_language() -> Result<String, JsValue> {
    let window = window().expect("Error occured! Are you calling this function in the server?");
    let navigator = window.navigator();

    if let Some(language) = navigator.language() {
        info!("Language: {}", language);
        return Ok(language);
    } else {
        error!("Error occured whilst getting language!");
        return Err(JsValue::from("Error occured whilst getting language!"));
    }
}

#[cfg(test)]
mod language_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn get_language_test() {
        let language = get_language().await;
        assert!(language.is_ok());
    }
}