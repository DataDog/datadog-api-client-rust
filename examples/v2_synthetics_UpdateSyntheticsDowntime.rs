// Update a Synthetics downtime returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SyntheticsDowntimeDataAttributesRequest;
use datadog_api_client::datadogV2::model::SyntheticsDowntimeDataRequest;
use datadog_api_client::datadogV2::model::SyntheticsDowntimeRequest;
use datadog_api_client::datadogV2::model::SyntheticsDowntimeResourceType;
use datadog_api_client::datadogV2::model::SyntheticsDowntimeTimeSlotDate;
use datadog_api_client::datadogV2::model::SyntheticsDowntimeTimeSlotRequest;

#[tokio::main]
async fn main() {
    let body = SyntheticsDowntimeRequest::new(SyntheticsDowntimeDataRequest::new(
        SyntheticsDowntimeDataAttributesRequest::new(
            true,
            "Weekly maintenance".to_string(),
            vec!["abc-def-123".to_string()],
            vec![SyntheticsDowntimeTimeSlotRequest::new(
                3600,
                SyntheticsDowntimeTimeSlotDate::new(15, 10, 30, 1, 2024),
                "Europe/Paris".to_string(),
            )],
        ),
        SyntheticsDowntimeResourceType::DOWNTIME,
    ));
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .update_synthetics_downtime("00000000-0000-0000-0000-000000000001".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
