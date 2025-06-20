// Create a Microsoft Sentinel custom destination returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI;
use datadog_api_client::datadogV2::model::CustomDestinationAttributeTagsRestrictionListType;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequest;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequestAttributes;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequestDefinition;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestination;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationMicrosoftSentinel;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationMicrosoftSentinelType;
use datadog_api_client::datadogV2::model::CustomDestinationType;

#[tokio::main]
async fn main() {
    let body =
        CustomDestinationCreateRequest
        ::new().data(
            CustomDestinationCreateRequestDefinition::new(
                CustomDestinationCreateRequestAttributes::new(
                    CustomDestinationForwardDestination::CustomDestinationForwardDestinationMicrosoftSentinel(
                        Box::new(
                            CustomDestinationForwardDestinationMicrosoftSentinel::new(
                                "9a2f4d83-2b5e-429e-a35a-2b3c4182db71".to_string(),
                                "https://my-dce-5kyl.eastus-1.ingest.monitor.azure.com".to_string(),
                                "dcr-000a00a000a00000a000000aa000a0aa".to_string(),
                                "Custom-MyTable".to_string(),
                                "f3c9a8a1-4c2e-4d2e-b911-9f3c28c3c8b2".to_string(),
                                CustomDestinationForwardDestinationMicrosoftSentinelType::MICROSOFT_SENTINEL,
                            ),
                        ),
                    ),
                    "Nginx logs".to_string(),
                )
                    .enabled(false)
                    .forward_tags(false)
                    .forward_tags_restriction_list(vec!["datacenter".to_string(), "host".to_string()])
                    .forward_tags_restriction_list_type(CustomDestinationAttributeTagsRestrictionListType::ALLOW_LIST)
                    .query("source:nginx".to_string()),
                CustomDestinationType::CUSTOM_DESTINATION,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = LogsCustomDestinationsAPI::with_config(configuration);
    let resp = api.create_logs_custom_destination(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
