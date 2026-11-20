// Search remediation groups returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SearchRemediationGroupsOptionalParams;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.search_remediation_groups", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let response = api.search_remediation_groups_with_pagination(
        SearchRemediationGroupsOptionalParams::default().page_size(2),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
