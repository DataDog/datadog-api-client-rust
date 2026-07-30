// List Elastic Cloud CCM accounts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_cloud_cost_management::ElasticCloudCloudCostManagementAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListElasticCloudCcmAccounts", true);
    let api = ElasticCloudCloudCostManagementAPI::with_config(configuration);
    let resp = api.list_elastic_cloud_ccm_accounts().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
