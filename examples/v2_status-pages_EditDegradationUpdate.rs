// Edit degradation update returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::EditDegradationUpdateOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::PatchDegradationUpdateRequest;
use datadog_api_client::datadogV2::model::PatchDegradationUpdateRequestData;
use datadog_api_client::datadogV2::model::PatchDegradationUpdateRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchDegradationUpdateRequestDataAttributesStatus;
use datadog_api_client::datadogV2::model::PatchDegradationUpdateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = PatchDegradationUpdateRequest::new().data(
        PatchDegradationUpdateRequestData::new(
            PatchDegradationUpdateRequestDataType::DEGRADATION_UPDATES,
        )
        .attributes(
            PatchDegradationUpdateRequestDataAttributes::new()
                .description(
                    "We've identified the source of the latency increase and are deploying a fix."
                        .to_string(),
                )
                .status(PatchDegradationUpdateRequestDataAttributesStatus::IDENTIFIED),
        )
        .id("00000000-0000-0000-0000-000000000000".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .edit_degradation_update(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            EditDegradationUpdateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
