// Create backfilled degradation returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateBackfilledDegradationOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateBackfilledDegradationRequest;
use datadog_api_client::datadogV2::model::CreateBackfilledDegradationRequestData;
use datadog_api_client::datadogV2::model::CreateBackfilledDegradationRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateBackfilledDegradationRequestDataAttributesUpdatesItems;
use datadog_api_client::datadogV2::model::CreateBackfilledDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems;
use datadog_api_client::datadogV2::model::CreateDegradationRequestDataAttributesStatus;
use datadog_api_client::datadogV2::model::PatchDegradationRequestDataType;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_attributes_components_0_components_0_id = uuid::Uuid::parse_str(
        &std::env::var("STATUS_PAGE_DATA_ATTRIBUTES_COMPONENTS_0_COMPONENTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let status_page_data_id = uuid::Uuid::parse_str(&std::env::var("STATUS_PAGE_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body =
        CreateBackfilledDegradationRequest
        ::new().data(
            CreateBackfilledDegradationRequestData::new(
                PatchDegradationRequestDataType::DEGRADATIONS,
            ).attributes(
                CreateBackfilledDegradationRequestDataAttributes::new(
                    "Past API Outage".to_string(),
                    vec![
                        CreateBackfilledDegradationRequestDataAttributesUpdatesItems::new(
                            DateTime::parse_from_rfc3339("2021-11-11T10:11:11+00:00")
                                .expect("Failed to parse datetime")
                                .with_timezone(&Utc),
                        )
                            .components_affected(
                                vec![
                                    CreateBackfilledDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems
                                    ::new(
                                        status_page_data_attributes_components_0_components_0_id.clone(),
                                        PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus
                                        ::DEGRADED,
                                    )
                                ],
                            )
                            .description("We detected elevated error rates in the API.".to_string())
                            .status(CreateDegradationRequestDataAttributesStatus::INVESTIGATING),
                        CreateBackfilledDegradationRequestDataAttributesUpdatesItems::new(
                            DateTime::parse_from_rfc3339("2021-11-11T10:41:11+00:00")
                                .expect("Failed to parse datetime")
                                .with_timezone(&Utc),
                        )
                            .components_affected(
                                vec![
                                    CreateBackfilledDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems
                                    ::new(
                                        status_page_data_attributes_components_0_components_0_id.clone(),
                                        PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus
                                        ::DEGRADED,
                                    )
                                ],
                            )
                            .description("Root cause identified as a misconfigured deployment.".to_string())
                            .status(CreateDegradationRequestDataAttributesStatus::IDENTIFIED),
                        CreateBackfilledDegradationRequestDataAttributesUpdatesItems::new(
                            DateTime::parse_from_rfc3339("2021-11-11T11:11:11+00:00")
                                .expect("Failed to parse datetime")
                                .with_timezone(&Utc),
                        )
                            .components_affected(
                                vec![
                                    CreateBackfilledDegradationRequestDataAttributesUpdatesItemsComponentsAffectedItems
                                    ::new(
                                        status_page_data_attributes_components_0_components_0_id.clone(),
                                        PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus
                                        ::OPERATIONAL,
                                    )
                                ],
                            )
                            .description("The issue has been resolved and API is operating normally.".to_string())
                            .status(CreateDegradationRequestDataAttributesStatus::RESOLVED)
                    ],
                ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_backfilled_degradation(
            status_page_data_id.clone(),
            body,
            CreateBackfilledDegradationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
