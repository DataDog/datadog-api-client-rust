// Convert security monitoring resource to Terraform returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformConvertAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformConvertData;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformConvertRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformResourceType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SecurityMonitoringTerraformConvertRequest::new(
        SecurityMonitoringTerraformConvertData::new(
            SecurityMonitoringTerraformConvertAttributes::new(BTreeMap::from([
                ("enabled".to_string(), Value::from("True")),
                (
                    "name".to_string(),
                    Value::from("Example-Security-Monitoring"),
                ),
                ("rule_query".to_string(), Value::from("source:cloudtrail")),
                ("suppression_query".to_string(), Value::from("env:test")),
            ])),
            "abc-123".to_string(),
            "convert_resource".to_string(),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.ConvertSecurityMonitoringTerraformResource", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .convert_security_monitoring_terraform_resource(
            SecurityMonitoringTerraformResourceType::SUPPRESSIONS,
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
