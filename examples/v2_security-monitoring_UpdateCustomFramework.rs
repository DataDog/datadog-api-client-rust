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
            "".to_string(),
            "".to_string(),
            vec![CustomFrameworkRequirement::new(
                vec![CustomFrameworkControl::new(
                    "".to_string(),
                    vec!["".to_string()],
                )],
                "".to_string(),
            )],
            "".to_string(),
        ),
        CustomFrameworkType::CUSTOM_FRAMEWORK,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateCustomFramework", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .update_custom_framework("handle".to_string(), "version".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
