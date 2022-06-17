use reqwasm::http::{Request, RequestMode};
use wasm_bindgen::UnwrapThrowExt;

pub struct TestService;

impl TestService {
    pub async fn make_request(url: &str, first: bool) {
        if !first {
            return;
        }

        let resp = Request::get(url)
            .mode(RequestMode::NoCors)
            .send()
            .await
            .unwrap_throw();

        let text = resp.text().await.unwrap();

        log::info!("{}", text);
    }
}
