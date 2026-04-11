// Export security monitoring resource to Terraform returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformResourceType;

#[tokio::main]
async fn main() {
    // there is a valid "suppression" in the system
    let suppression_data_id = std::env::var("SUPPRESSION_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.ExportSecurityMonitoringTerraformResource", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .export_security_monitoring_terraform_resource(
            SecurityMonitoringTerraformResourceType::SUPPRESSIONS,
            suppression_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
