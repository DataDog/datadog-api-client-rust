// Update App Self-Service Status returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppSelfServiceType;
use datadog_api_client::datadogV2::model::UpdateAppSelfServiceRequest;
use datadog_api_client::datadogV2::model::UpdateAppSelfServiceRequestData;
use datadog_api_client::datadogV2::model::UpdateAppSelfServiceRequestDataAttributes;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpdateAppSelfServiceRequest::new().data(
        UpdateAppSelfServiceRequestData::new()
            .attributes(UpdateAppSelfServiceRequestDataAttributes::new(true))
            .type_(AppSelfServiceType::SELFSERVICE),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api
        .update_app_self_service(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
