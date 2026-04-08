// Update a Splunk custom destination's destination preserves the sourcetype
// returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestination;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationSplunk;
use datadog_api_client::datadogV2::model::CustomDestinationForwardDestinationSplunkType;
use datadog_api_client::datadogV2::model::CustomDestinationType;
use datadog_api_client::datadogV2::model::CustomDestinationUpdateRequest;
use datadog_api_client::datadogV2::model::CustomDestinationUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::CustomDestinationUpdateRequestDefinition;

#[tokio::main]
async fn main() {
    // there is a valid "custom_destination_splunk_with_sourcetype" in the system
    let custom_destination_splunk_with_sourcetype_data_id =
        std::env::var("CUSTOM_DESTINATION_SPLUNK_WITH_SOURCETYPE_DATA_ID").unwrap();
    let body = CustomDestinationUpdateRequest::new().data(
        CustomDestinationUpdateRequestDefinition::new(
            custom_destination_splunk_with_sourcetype_data_id.clone(),
            CustomDestinationType::CUSTOM_DESTINATION,
        )
        .attributes(
            CustomDestinationUpdateRequestAttributes::new().forwarder_destination(
                CustomDestinationForwardDestination::CustomDestinationForwardDestinationSplunk(
                    Box::new(CustomDestinationForwardDestinationSplunk::new(
                        "my-access-token".to_string(),
                        "https://updated-example.com".to_string(),
                        CustomDestinationForwardDestinationSplunkType::SPLUNK_HEC,
                    )),
                ),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = LogsCustomDestinationsAPI::with_config(configuration);
    let resp = api
        .update_logs_custom_destination(
            custom_destination_splunk_with_sourcetype_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
