// Update resource in Confluent account returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_confluent_cloud::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        ConfluentResourceRequest::new(
            ConfluentResourceRequestData::new(
                ConfluentResourceRequestAttributes::new("kafka".to_string())
                    .enable_custom_metrics(false)
                    .tags(vec!["myTag".to_string(), "myTag2:myValue".to_string()]),
                "resource-id-123".to_string(),
                ConfluentResourceType::CONFLUENT_CLOUD_RESOURCES,
            ),
        );
    let configuration = Configuration::new();
    let api = ConfluentCloudAPI::with_config(configuration);
    let resp = api.update_confluent_resource("account_id".to_string(), "resource_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
