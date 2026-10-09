// Apply a severity override to security findings returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::SeverityOverrideAttributes;
use datadog_api_client::datadogV2::model::SeverityOverrideDataType;
use datadog_api_client::datadogV2::model::SeverityOverrideRequest;
use datadog_api_client::datadogV2::model::SeverityOverrideRequestData;
use datadog_api_client::datadogV2::model::SeverityOverrideRequestDataAttributes;
use datadog_api_client::datadogV2::model::SeverityOverrideRequestDataRelationships;
use datadog_api_client::datadogV2::model::SeverityOverrideSet;
use datadog_api_client::datadogV2::model::SeverityOverrideSetActionType;
use datadog_api_client::datadogV2::model::SeverityOverrideValue;

#[tokio::main]
async fn main() {
    let body = SeverityOverrideRequest::new(SeverityOverrideRequestData::new(
        SeverityOverrideRequestDataAttributes::new(
            SeverityOverrideAttributes::SeverityOverrideSet(Box::new(
                SeverityOverrideSet::new(
                    SeverityOverrideSetActionType::SET,
                    SeverityOverrideValue::HIGH,
                )
                .description("Database contains sensitive data.".to_string()),
            )),
        ),
        SeverityOverrideRequestDataRelationships::new(Findings::new().data(vec![
            FindingData::new(
                "ZGVmLTAwMC0wYmd-MDE4NjcyMDJkMzE4MDE5ODY5MGE4ZmQ2MmFlMjg0Y2M=".to_string(),
                FindingDataType::FINDINGS,
            ),
        ])),
        SeverityOverrideDataType::SEVERITY_OVERRIDE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_findings_severity", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.update_findings_severity(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
