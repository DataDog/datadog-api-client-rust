// List test examples returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_examples::TestExamplesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = TestExamplesAPI::with_config(configuration);
    let resp = api.list_test_examples().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
