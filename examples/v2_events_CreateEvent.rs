// Post a change event returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::ChangeEvent;
use datadog_api_client::datadogV2::model::ChangeEventCategory;
use datadog_api_client::datadogV2::model::ChangeEventCreateRequest;
use datadog_api_client::datadogV2::model::ChangeEventCreateRequestType;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributes;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesAuthor;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesAuthorType;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesChangedResource;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesChangedResourceName;

#[tokio::main]
async fn main() {
    let body = ChangeEventCreateRequest::new()
        .attributes(
            ChangeEvent::new(
                ChangeEventCustomAttributes::new(ChangeEventCustomAttributesChangedResource::new(
                    ChangeEventCustomAttributesChangedResourceName::FEATURE_FLAG,
                    "".to_string(),
                ))
                .author(ChangeEventCustomAttributesAuthor::new(
                    "".to_string(),
                    ChangeEventCustomAttributesAuthorType::USER,
                )),
                ChangeEventCategory::CHANGE,
                "payment_processed feature flag updated".to_string(),
            )
            .message("payment_processed feature flag has been enabled".to_string())
            .tags(vec!["environment:test".to_string()])
            .timestamp("2017-01-15T01:30:15.010000Z".to_string()),
        )
        .type_(ChangeEventCreateRequestType::EVENT);
    let configuration = datadog::Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.create_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
