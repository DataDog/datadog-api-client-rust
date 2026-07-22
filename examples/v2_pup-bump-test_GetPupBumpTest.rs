// Get pup bump test resource returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_pup_bump_test::PupBumpTestAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetPupBumpTest", true);
    let api = PupBumpTestAPI::with_config(configuration);
    let resp = api.get_pup_bump_test().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
