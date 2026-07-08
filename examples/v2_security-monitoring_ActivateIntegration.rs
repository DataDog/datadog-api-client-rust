// Activate an entity context sync integration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::ActivateIntegrationOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationActivateAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationActivateData;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationActivateRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringIntegrationActivateResourceType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringIntegrationActivateRequest::new().data(
        SecurityMonitoringIntegrationActivateData::new()
            .attributes(
                SecurityMonitoringIntegrationActivateAttributes::new()
                    .domain("default".to_string())
                    .name("My Entra ID Integration".to_string())
                    .settings(BTreeMap::from([(
                        "setting1".to_string(),
                        Value::from("value1"),
                    )])),
            )
            .type_(SecurityMonitoringIntegrationActivateResourceType::ACTIVATE_ENTRA_ID_REQUEST),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ActivateIntegration", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .activate_integration(
            "entra_id".to_string(),
            ActivateIntegrationOptionalParams::default().body(body),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
