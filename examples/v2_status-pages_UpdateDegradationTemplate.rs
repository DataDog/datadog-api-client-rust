// Update degradation template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::api_status_pages::UpdateDegradationTemplateOptionalParams;
use datadog_api_client::datadogV2::model::CreateDegradationRequestDataAttributesStatus;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequest;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestData;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataAttributesComponentsAffectedItems;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataAttributesUpdatesItems;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        PatchDegradationTemplateRequest
        ::new().data(
            PatchDegradationTemplateRequestData::new(
                "".to_string(),
                PatchDegradationTemplateRequestDataType::DEGRADATION_TEMPLATES,
            ).attributes(
                PatchDegradationTemplateRequestDataAttributes::new()
                    .components_affected(
                        vec![
                            PatchDegradationTemplateRequestDataAttributesComponentsAffectedItems::new(
                                "".to_string(),
                                PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus
                                ::OPERATIONAL,
                            )
                        ],
                    )
                    .updates(
                        vec![
                            PatchDegradationTemplateRequestDataAttributesUpdatesItems::new(
                                CreateDegradationRequestDataAttributesStatus::INVESTIGATING,
                            )
                        ],
                    ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .update_degradation_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            UpdateDegradationTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
