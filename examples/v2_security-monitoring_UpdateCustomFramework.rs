// Update a custom framework returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CustomFrameworkControl;
use datadog_api_client::datadogV2::model::CustomFrameworkData;
use datadog_api_client::datadogV2::model::CustomFrameworkDataAttributes;
use datadog_api_client::datadogV2::model::CustomFrameworkRequirement;
use datadog_api_client::datadogV2::model::CustomFrameworkType;
use datadog_api_client::datadogV2::model::UpdateCustomFrameworkRequest;

#[tokio::main]
async fn main() {
    let body = UpdateCustomFrameworkRequest::new(CustomFrameworkData::new(
        CustomFrameworkDataAttributes::new(
            "create-framework-new".to_string(),
            "name".to_string(),
            vec![CustomFrameworkRequirement::new(
                vec![CustomFrameworkControl::new(
                    "control".to_string(),
                    vec!["def-000-be9".to_string()],
                )],
                "requirement".to_string(),
            )],
            "10".to_string(),
        )
        .icon_url("test-url".to_string()),
        CustomFrameworkType::CUSTOM_FRAMEWORK,
    ));
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_custom_framework("create-framework-new".to_string(), "10".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
