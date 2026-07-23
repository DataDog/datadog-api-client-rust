// Create degradation template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateDegradationTemplateOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateDegradationRequestDataAttributesStatus;
use datadog_api_client::datadogV2::model::CreateDegradationTemplateRequest;
use datadog_api_client::datadogV2::model::CreateDegradationTemplateRequestData;
use datadog_api_client::datadogV2::model::CreateDegradationTemplateRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateDegradationTemplateRequestDataAttributesComponentsAffectedItems;
use datadog_api_client::datadogV2::model::CreateDegradationTemplateRequestDataAttributesUpdatesItems;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus;
use datadog_api_client::datadogV2::model::PatchDegradationTemplateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        CreateDegradationTemplateRequest
        ::new().data(
            CreateDegradationTemplateRequestData::new(
                PatchDegradationTemplateRequestDataType::DEGRADATION_TEMPLATES,
            ).attributes(
                CreateDegradationTemplateRequestDataAttributes::new("".to_string())
                    .components_affected(
                        vec![
                            CreateDegradationTemplateRequestDataAttributesComponentsAffectedItems::new(
                                "".to_string(),
                                PatchDegradationTemplateRequestDataAttributesComponentsAffectedItemsStatus
                                ::OPERATIONAL,
                            )
                        ],
                    )
                    .updates(
                        vec![
                            CreateDegradationTemplateRequestDataAttributesUpdatesItems::new(
                                CreateDegradationRequestDataAttributesStatus::INVESTIGATING,
                            )
                        ],
                    ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_degradation_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            CreateDegradationTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
