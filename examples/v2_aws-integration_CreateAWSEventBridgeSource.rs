// Create an Amazon EventBridge source returns "Amazon EventBridge source
// created." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_aws_integration::AWSIntegrationAPI;
use datadog_api_client::datadogV2::model::AWSEventBridgeCreateRequest;
use datadog_api_client::datadogV2::model::AWSEventBridgeCreateRequestAttributes;
use datadog_api_client::datadogV2::model::AWSEventBridgeCreateRequestData;
use datadog_api_client::datadogV2::model::AWSEventBridgeType;

#[tokio::main]
async fn main() {
    let body = AWSEventBridgeCreateRequest::new(AWSEventBridgeCreateRequestData::new(
        AWSEventBridgeCreateRequestAttributes::new(
            "123456789012".to_string(),
            "app-alerts".to_string(),
            "us-east-1".to_string(),
        )
        .create_event_bus(true),
        AWSEventBridgeType::EVENT_BRIDGE,
    ));
    let configuration = datadog::Configuration::new();
    let api = AWSIntegrationAPI::with_config(configuration);
    let resp = api.create_aws_event_bridge_source(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
