// Get widgets from an image returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_stegadography::StegadographyAPI;
use std::fs;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = StegadographyAPI::with_config(configuration);
    let resp = api
        .get_stegadography_widgets(fs::read("fixtures/stegadography/image.png").unwrap())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
