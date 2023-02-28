use wasm_bindgen::prelude::*;
use js_sys::Function;
use web_sys::{NetworkInformation, ConnectionType};

#[wasm_bindgen]
pub async fn is_online() -> Result<bool, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let online = navigator.on_line();

    if online {
      return Ok(true);
    } else {
      return Ok(false);
    }
}

/// Returns
#[wasm_bindgen]
pub async fn check_connectivity(online: &Function, offline: &Function) {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let is_online = navigator.on_line();

    if is_online {
      online.call0(&JsValue::NULL).unwrap();
    } else {
      offline.call0(&JsValue::NULL).unwrap();
    }
}

#[wasm_bindgen]
pub async fn get_network_information() -> Result<NetworkInformation, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let connection = navigator.connection().unwrap();

    Ok(connection)
}

#[wasm_bindgen]
pub async fn get_type() -> Result<ConnectionType, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let connection = navigator.connection().unwrap();

    let connection_type = connection.type_();

    Ok(connection_type)
}

#[cfg(test)]
mod connection_tests {
  use super::*;
  use wasm_bindgen_test::*;

  wasm_bindgen_test_configure!(run_in_browser);

  #[wasm_bindgen_test]
  async fn is_online_test() {
    let result = is_online().await;
    assert_eq!(result.is_ok(), true);
  }

  #[wasm_bindgen_test]
  async fn check_connectivity_test() {
    let online = Function::new_no_args("console.log(\"Online!\");");

    let offline = Function::new_no_args("console.log(\"Offline!\");");

    check_connectivity(&online, &offline).await;
  }

  #[wasm_bindgen_test]
  async fn get_network_information_test() {
    let result = get_network_information().await;
    assert_eq!(result.is_ok(), true);
  }

  #[wasm_bindgen_test]
  async fn get_type_test() {
    let result = get_type().await;
    assert_eq!(result.is_ok(), true);
    // You can additionally add another assertion for connection_type
    // with regards to what connection is used. I omitted it because 
    // not everyone accesses the internet the same way
    
    // assert_eq!(result.unwrap(), ConnectionType::Wifi)
  }
}