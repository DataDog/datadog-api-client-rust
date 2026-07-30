// Delete an Elastic Cloud CCM account returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteElasticCloudCcmAccount", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .delete_elastic_cloud_ccm_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
