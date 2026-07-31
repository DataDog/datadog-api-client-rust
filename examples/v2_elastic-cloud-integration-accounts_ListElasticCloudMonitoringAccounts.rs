// List Elastic Cloud monitoring accounts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListElasticCloudMonitoringAccounts", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api.list_elastic_cloud_monitoring_accounts().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
