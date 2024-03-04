// Patch a Synthetic test returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_api_test" in the system
    let synthetics_api_test_public_id = std::env::var("SYNTHETICS_API_TEST_PUBLIC_ID").unwrap();
    let body =
        SyntheticsPatchTestBody
        ::new().data(
            vec![
                SyntheticsPatchTestOperation::new()
                    .op(SyntheticsPatchTestOperationName::REPLACE)
                    .path("/name".to_string())
                    .value("New test name".to_string()),
                SyntheticsPatchTestOperation::new()
                    .op(SyntheticsPatchTestOperationName::REMOVE)
                    .path("/config/assertions/0".to_string())
            ],
        );
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.patch_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
