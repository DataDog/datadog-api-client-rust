// Export security monitoring resources to Terraform returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformBulkExportAttributes;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformBulkExportData;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformBulkExportRequest;
use datadog_api_client::datadogV2::model::SecurityMonitoringTerraformResourceType;

#[tokio::main]
async fn main() {
    // there is a valid "suppression" in the system
    let suppression_data_id = std::env::var("SUPPRESSION_DATA_ID").unwrap();
    let body = SecurityMonitoringTerraformBulkExportRequest::new(
        SecurityMonitoringTerraformBulkExportData::new(
            SecurityMonitoringTerraformBulkExportAttributes::new(vec![suppression_data_id.clone()]),
            "bulk_export_resources".to_string(),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.BulkExportSecurityMonitoringTerraformResources", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api
        .bulk_export_security_monitoring_terraform_resources(
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
