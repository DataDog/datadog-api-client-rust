// Get a Model Lab run returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_model_lab_api::ModelLabAPIAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetModelLabRun", true);
    let api = ModelLabAPIAPI::with_config(configuration);
    let resp = api.get_model_lab_run(70158).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
