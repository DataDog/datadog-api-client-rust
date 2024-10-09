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
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesChangedResourceType;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItems;
use datadog_api_client::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItemsType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = ChangeEventCreateRequest::new()
        .attributes(
            ChangeEvent::new(
                ChangeEventCustomAttributes::new(ChangeEventCustomAttributesChangedResource::new(
                    "fallback_payments_test".to_string(),
                    ChangeEventCustomAttributesChangedResourceType::FEATURE_FLAG,
                ))
                .author(ChangeEventCustomAttributesAuthor::new(
                    "".to_string(),
                    ChangeEventCustomAttributesAuthorType::USER,
                ))
                .change_metadata(BTreeMap::new())
                .impacted_resources(vec![
                    ChangeEventCustomAttributesImpactedResourcesItems::new(
                        "payments_api".to_string(),
                        ChangeEventCustomAttributesImpactedResourcesItemsType::SERVICE,
                    ),
                ])
                .new_value(BTreeMap::new())
                .prev_value(BTreeMap::new()),
                ChangeEventCategory::CHANGE,
                "payment_processed feature flag updated".to_string(),
            )
            .message("payment_processed feature flag has been enabled".to_string())
            .tags(vec!["environment:test".to_string()]),
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
