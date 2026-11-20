// Delete a Claude Compliance integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_claude_compliance_integration::ClaudeComplianceIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.delete_claude_compliance_integration_account", true);
    let api = ClaudeComplianceIntegrationAPI::with_config(configuration);
    let resp = api
        .delete_claude_compliance_integration_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
