// Update a RUM application returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "rum_application" in the system
    let rum_application_data_id = std::env::var("RUM_APPLICATION_DATA_ID").unwrap();
    let body =
        RUMApplicationUpdateRequest::new(
            RUMApplicationUpdate::new(
                rum_application_data_id,
                RUMApplicationUpdateType::RUM_APPLICATION_UPDATE,
            ).attributes(
                RUMApplicationUpdateAttributes::new()
                    .name("updated_name_for_my_existing_rum_application".to_string())
                    .type_("browser".to_string()),
            ),
        );
    let configuration = Configuration::new();
    let api = RUMAPI::with_config(configuration);
    let resp = api.update_rum_application(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
