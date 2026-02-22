// List all forms returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::api_forms::ListFormsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListForms", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api.list_forms(ListFormsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
