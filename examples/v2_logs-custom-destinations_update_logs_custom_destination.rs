// Update a custom destination returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_custom_destinations::LogsCustomDestinationsAPI;
use datadog_api_client::datadogV2::model::CustomDestinationType;
use datadog_api_client::datadogV2::model::CustomDestinationUpdateRequest;
use datadog_api_client::datadogV2::model::CustomDestinationUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::CustomDestinationUpdateRequestDefinition;

#[tokio::main]
async fn main() {
    // there is a valid "custom_destination" in the system
    let custom_destination_data_id = std::env::var("CUSTOM_DESTINATION_DATA_ID").unwrap();
    let body = CustomDestinationUpdateRequest::new().data(
        CustomDestinationUpdateRequestDefinition::new(
            custom_destination_data_id.clone(),
            CustomDestinationType::CUSTOM_DESTINATION,
        )
        .attributes(
            CustomDestinationUpdateRequestAttributes::new()
                .name("Nginx logs (Updated)".to_string()),
        ),
    );
    let configuration = Configuration::new();
    let api = LogsCustomDestinationsAPI::with_config(configuration);
    let resp = api
        .update_logs_custom_destination(custom_destination_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}