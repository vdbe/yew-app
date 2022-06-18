use reqwasm::http::{Request, RequestMode};
use wasm_bindgen::UnwrapThrowExt;

pub struct TestService;

impl TestService {
    pub async fn make_request(url: &str) {
        let fetched_videos: String = Request::get(url)
            .mode(RequestMode::Cors)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        log::info!("{:?}", fetched_videos);
    }
}
