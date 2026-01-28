// Create degradation returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateDegradationOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateDegradationRequest;
use datadog_api_client::datadogV2::model::CreateDegradationRequestData;
use datadog_api_client::datadogV2::model::CreateDegradationRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateDegradationRequestDataAttributesComponentsAffectedItems;
use datadog_api_client::datadogV2::model::CreateDegradationRequestDataAttributesStatus;
use datadog_api_client::datadogV2::model::PatchDegradationRequestDataType;
use datadog_api_client::datadogV2::model::StatusPagesComponentDataAttributesStatus;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_attributes_components_0_components_0_id = uuid::Uuid::parse_str(
        &std::env::var("STATUS_PAGE_DATA_ATTRIBUTES_COMPONENTS_0_COMPONENTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let status_page_data_id = uuid::Uuid::parse_str(&std::env::var("STATUS_PAGE_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body = CreateDegradationRequest::new().data(CreateDegradationRequestData::new(
        CreateDegradationRequestDataAttributes::new(
            vec![
                CreateDegradationRequestDataAttributesComponentsAffectedItems::new(
                    status_page_data_attributes_components_0_components_0_id.clone(),
                    StatusPagesComponentDataAttributesStatus::MAJOR_OUTAGE,
                ),
            ],
            CreateDegradationRequestDataAttributesStatus::INVESTIGATING,
            "Elevated API Latency".to_string(),
        )
        .description(
            "Our API is experiencing elevated latency. We are investigating the issue.".to_string(),
        ),
        PatchDegradationRequestDataType::DEGRADATIONS,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_degradation(
            status_page_data_id.clone(),
            body,
            CreateDegradationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
