// Update App Protection Level returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppProtectionLevel;
use datadog_api_client::datadogV2::model::AppProtectionLevelType;
use datadog_api_client::datadogV2::model::UpdateAppProtectionLevelRequest;
use datadog_api_client::datadogV2::model::UpdateAppProtectionLevelRequestData;
use datadog_api_client::datadogV2::model::UpdateAppProtectionLevelRequestDataAttributes;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpdateAppProtectionLevelRequest::new().data(
        UpdateAppProtectionLevelRequestData::new()
            .attributes(UpdateAppProtectionLevelRequestDataAttributes::new(
                AppProtectionLevel::APPROVAL_REQUIRED,
            ))
            .type_(AppProtectionLevelType::PROTECTIONLEVEL),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api
        .update_protection_level(
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
