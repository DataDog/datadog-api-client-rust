// Delete an Amazon EventBridge source returns "Amazon EventBridge source
// deleted." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSEventBridgeDeleteRequest;
use datadog_api_client::datadogV2::model::AWSEventBridgeDeleteRequestAttributes;
use datadog_api_client::datadogV2::model::AWSEventBridgeDeleteRequestData;
use datadog_api_client::datadogV2::model::AWSEventBridgeType;

#[tokio::main]
async fn main() {
    let body = AWSEventBridgeDeleteRequest::new(AWSEventBridgeDeleteRequestData::new(
        AWSEventBridgeDeleteRequestAttributes::new(
            "123456789012".to_string(),
            "app-alerts-zyxw3210".to_string(),
            "us-east-1".to_string(),
        ),
        AWSEventBridgeType::EVENT_BRIDGE,
    ));
    let configuration = datadog::Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.delete_aws_event_bridge_source(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
