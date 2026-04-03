// Create a Splunk custom destination with an empty string sourcetype returns "OK"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequest;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequestAttributes;
use datadog_api_client::datadogV2::model::CustomDestinationCreateRequestDefinition;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestination;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationSplunk;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationSplunkType;
use datadog_api_client::datadogV2::model::CustomDestinationType;

#[tokio::main]
async fn main() {
    let body =
        CustomDestinationCreateRequest::new().data(CustomDestinationCreateRequestDefinition::new(
            CustomDestinationCreateRequestAttributes::new(
                CustomDestinationForwardDestination::CustomDestinationForwardDestinationSplunk(
                    Box::new(
                        CustomDestinationForwardDestinationSplunk::new(
                            "my-access-token".to_string(),
                            "https://example.com".to_string(),
                            CustomDestinationForwardDestinationSplunkType::SPLUNK_HEC,
                        )
                        .sourcetype(Some("".to_string())),
                    ),
                ),
                "Nginx logs".to_string(),
            )
            .enabled(false)
            .forward_tags(false)
            .query("source:nginx".to_string()),
            CustomDestinationType::CUSTOM_DESTINATION,
        ));
    let configuration = datadog::Configuration::new();
    let api = LogsCustomDestinationsAPI::with_config(configuration);
    let resp = api.create_logs_custom_destination(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
