// Get a Mobile test returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_mobile_test" in the system
    let synthetics_mobile_test_public_id =
        std::env::var("SYNTHETICS_MOBILE_TEST_PUBLIC_ID").unwrap();

    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .get_mobile_test(synthetics_mobile_test_public_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
