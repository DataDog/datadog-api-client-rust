// Post an event returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_events::EventsAPI;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributes;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesAuthor;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesAuthorType;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesChangedResource;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesChangedResourceType;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItems;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItemsType;
use datadog_api_client::datadogV2::model::EventCategory;
use datadog_api_client::datadogV2::model::EventCreateRequest;
use datadog_api_client::datadogV2::model::EventCreateRequestPayload;
use datadog_api_client::datadogV2::model::EventCreateRequestType;
use datadog_api_client::datadogV2::model::EventPayload;
use datadog_api_client::datadogV2::model::EventPayloadAttributes;
use datadog_api_client::datadogV2::model::EventPayloadIntegrationId;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = EventCreateRequestPayload::new(EventCreateRequest::new(
        EventPayload::new(
            EventPayloadAttributes::ChangeEventCustomAttributes(Box::new(
                ChangeEventCustomAttributes::new(ChangeEventCustomAttributesChangedResource::new(
                    "fallback_payments_test".to_string(),
                    ChangeEventCustomAttributesChangedResourceType::FEATURE_FLAG,
                ))
                .author(ChangeEventCustomAttributesAuthor::new(
                    "datadog@datadog.com".to_string(),
                    ChangeEventCustomAttributesAuthorType::USER,
                ))
                .change_metadata(BTreeMap::from([(
                    "resource_link".to_string(),
                    Value::from("datadog.com/feature/fallback_payments_test"),
                )]))
                .impacted_resources(vec![
                    ChangeEventCustomAttributesImpactedResourcesItems::new(
                        "payments_api".to_string(),
                        ChangeEventCustomAttributesImpactedResourcesItemsType::SERVICE,
                    ),
                ])
                .new_value(BTreeMap::from([
                    ("enabled".to_string(), Value::from("True")),
                    ("percentage".to_string(), Value::from("50%")),
                ]))
                .prev_value(BTreeMap::from([
                    ("enabled".to_string(), Value::from("True")),
                    ("percentage".to_string(), Value::from("10%")),
                ])),
            )),
            EventCategory::CHANGE,
            "Datadog api client test".to_string(),
        )
        .aggregation_key("aggregation_key_123".to_string())
        .integration_id(EventPayloadIntegrationId::CUSTOM_EVENTS)
        .message("payment_processed feature flag has been enabled".to_string())
        .tags(vec!["env:test".to_string()]),
        EventCreateRequestType::EVENT,
    ));
    let configuration = datadog::Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp = api.create_event(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
