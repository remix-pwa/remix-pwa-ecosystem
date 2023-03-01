use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::Geolocation;

/// Returns a `Geolocation` object that allows you to 
/// determine the position of the user's device programatically.
/// 
/// *Requires the geolocation permission*
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)
#[wasm_bindgen(js_name = "getGeolocationObject")]
pub async fn get_geolocation_object() -> Result<Geolocation, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let geolocation = navigator.geolocation().unwrap();

    Ok(geolocation)
}

// todo!("Add geolocation getPosition but with error callback and also with options");

/// Returns the current position of the user's device. Takes in a 
/// success callback that would be incoked when the user's device 
/// has been successfully located
/// 
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)
#[wasm_bindgen(js_name = "getCurrentPosition")]
pub async fn get_current_position(success_callback: &Function) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let geolocation = navigator.geolocation().unwrap();
    match geolocation.get_current_position(success_callback) {
        Ok(_) => {
          return Ok(());
        },
        Err(e) => {
            log::error!("Error getting current position: {:?}", e);
            return Err(e);
        }
    };
}

#[cfg(test)]
mod geolocation_tests {
    use std::ops::Not;

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn get_geolocation_object_test() {
        let geolocation = get_geolocation_object().await.unwrap();

        assert_eq!(geolocation.is_null().not(), true);
    }

    #[wasm_bindgen_test]
    async fn get_current_position_test() {
        let success_callback = Function::new_no_args("console.log('Woohoo! We are finally live somewhere in the world...')");

        let result = get_current_position(&success_callback).await;

        assert_eq!(result.is_ok(), true);
    }
}