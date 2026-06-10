// Get a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::api_forms::GetFormOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "form" in the system
    let form_data_id =
        uuid::Uuid::parse_str(&std::env::var("FORM_DATA_ID").unwrap()).expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .get_form(form_data_id.clone(), GetFormOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
