// Enable Storage Management for a bucket returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_inventory_sync_configs::CloudInventorySyncConfigsAPI;
use datadog_api_client::datadogV2::model::CloudInventoryCloudProviderId;
use datadog_api_client::datadogV2::model::CloudInventoryCloudProviderRequestType;
use datadog_api_client::datadogV2::model::CloudInventorySyncConfigAWSRequestAttributes;
use datadog_api_client::datadogV2::model::CloudInventorySyncConfigAzureRequestAttributes;
use datadog_api_client::datadogV2::model::CloudInventorySyncConfigGCPRequestAttributes;
use datadog_api_client::datadogV2::model::UpsertCloudInventorySyncConfigRequest;
use datadog_api_client::datadogV2::model::UpsertCloudInventorySyncConfigRequestAttributes;
use datadog_api_client::datadogV2::model::UpsertCloudInventorySyncConfigRequestData;

#[tokio::main]
async fn main() {
    let body =
        UpsertCloudInventorySyncConfigRequest::new(UpsertCloudInventorySyncConfigRequestData::new(
            UpsertCloudInventorySyncConfigRequestAttributes::new()
                .aws(
                    CloudInventorySyncConfigAWSRequestAttributes::new(
                        "123456789012".to_string(),
                        "my-inventory-bucket".to_string(),
                        "us-east-1".to_string(),
                    )
                    .destination_prefix("logs/".to_string()),
                )
                .azure(CloudInventorySyncConfigAzureRequestAttributes::new(
                    "11111111-1111-1111-1111-111111111111".to_string(),
                    "inventory-container".to_string(),
                    "my-resource-group".to_string(),
                    "mystorageaccount".to_string(),
                    "33333333-3333-3333-3333-333333333333".to_string(),
                    "22222222-2222-2222-2222-222222222222".to_string(),
                ))
                .gcp(CloudInventorySyncConfigGCPRequestAttributes::new(
                    "my-inventory-reports".to_string(),
                    "my-gcp-project".to_string(),
                    "reader@my-gcp-project.iam.gserviceaccount.com".to_string(),
                    "my-monitored-bucket".to_string(),
                )),
            CloudInventoryCloudProviderId::AWS,
            CloudInventoryCloudProviderRequestType::CLOUD_PROVIDER,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertSyncConfig", true);
    let api = CloudInventorySyncConfigsAPI::with_config(configuration);
    let resp = api.upsert_sync_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
