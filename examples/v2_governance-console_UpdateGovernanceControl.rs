// Update a control returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_governance_console::GovernanceConsoleAPI;
use datadog_api_client::datadogV2::model::GovernanceControlResourceType;
use datadog_api_client::datadogV2::model::GovernanceControlUpdateAttributes;
use datadog_api_client::datadogV2::model::GovernanceControlUpdateData;
use datadog_api_client::datadogV2::model::GovernanceControlUpdateRequest;

#[tokio::main]
async fn main() {
    let body = GovernanceControlUpdateRequest::new(
        GovernanceControlUpdateData::new(GovernanceControlResourceType::GOVERNANCE_CONTROL)
            .attributes(
                GovernanceControlUpdateAttributes::new()
                    .detection_frequency("daily".to_string())
                    .mitigation_type("revoke_api_key".to_string()),
            ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateGovernanceControl", true);
    let api = GovernanceConsoleAPI::with_config(configuration);
    let resp = api
        .update_governance_control("detection_type".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
