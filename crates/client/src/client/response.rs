use wasm_bindgen::prelude::*;

/// An object denoting wether a response is successful 
/// or bad
/// 
/// ## Attributes
/// `status`: "success" | "error"
/// 
/// `message`: `string`
/// 
/// ## Methods
/// 
/// `ClientResponse.getStatus(): string`: Returns the status message
/// 
/// `ClientResponse.getMessage(): string`: Returns the message
/// 
/// [Source](https://github.com/remix-pwa/remix-pwa-ecosystem/blob/main/crates/client/src/client/response.rs)
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ClientResponse {
  status: String,
  message: String,
}

#[wasm_bindgen]
impl ClientResponse {
  #[wasm_bindgen(constructor)]
  pub fn new(status: &str, message: &str) -> ClientResponse {
    ClientResponse { status: status.to_string(), message: message.to_string() }
  }

  #[wasm_bindgen(getter, js_name = "getStatus")]
  pub fn get_status(&self) -> String {
    self.status.clone()
  }

  #[wasm_bindgen(getter, js_name = "getMessage")]
  pub fn get_message(&self) -> String {
    self.message.clone()
  }
}

#[cfg(test)]
mod client_response_tests {
  use super::*;

  #[test]
  fn new_test() {
    let status = "success";
    let message = "Hello, World!";
    let client_response = ClientResponse::new(status, message);
    assert_eq!(client_response.get_status(), status.to_string());
    assert_eq!(client_response.get_message(), message.to_string());
  }

  #[test]
  fn get_status_test() {
    let status = "success";
    let message = "Hello, World!";
    let client_response = ClientResponse::new(status, message);
    assert_eq!(client_response.get_status(), status.to_string());
  }

  #[test]
  fn get_message_test() {
    let status = "success";
    let message = "Hello, World!";
    let client_response = ClientResponse::new(status, message);
    assert_eq!(client_response.get_message(), message.to_string());
  }
}