// Search security findings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::SecurityFindingsSearchRequest;
use datadog_api_client::datadogV2::model::SecurityFindingsSearchRequestData;
use datadog_api_client::datadogV2::model::SecurityFindingsSearchRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = SecurityFindingsSearchRequest::new().data(
        SecurityFindingsSearchRequestData::new().attributes(
            SecurityFindingsSearchRequestDataAttributes::new()
                .filter("@severity:(critical OR high)".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.search_security_findings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
