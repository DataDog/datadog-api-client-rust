// Get a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::api_forms::GetFormOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .get_form(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            GetFormOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
