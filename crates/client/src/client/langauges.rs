use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Returns the languages of the browser in an array
/// that's ordered by user preference.
/// 
/// ## Example
/// 
/// ```tsx
/// let languages: string[] = await getLanguages();
/// setUserLanguages(languages)
/// ```
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)
#[wasm_bindgen(js_name = "getLanguages")]
pub async fn get_languages() -> Result<Array, JsValue> {
    let languages = web_sys::window()
        .expect("Can't find the window object. Ensure you are calling this API in the browser")
        .navigator()
        .languages();

    let languages_vec = languages.iter().map(|l| l.as_string().unwrap()).collect::<Vec<String>>();
    log::info!("Languages: {:?}", languages_vec);
    Ok(languages)
}

#[cfg(test)]
mod languages_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn get_languages_test() {
        let languages = get_languages().await;
        assert!(languages.is_ok());
    }
}