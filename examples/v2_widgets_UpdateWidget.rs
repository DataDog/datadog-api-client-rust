// Update a widget returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_widgets::WidgetsAPI;
use datadog_api_client::datadogV2::model::CreateOrUpdateWidgetRequest;
use datadog_api_client::datadogV2::model::CreateOrUpdateWidgetRequestAttributes;
use datadog_api_client::datadogV2::model::CreateOrUpdateWidgetRequestData;
use datadog_api_client::datadogV2::model::WidgetDefinition;
use datadog_api_client::datadogV2::model::WidgetExperienceType;
use datadog_api_client::datadogV2::model::WidgetType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateOrUpdateWidgetRequest::new(CreateOrUpdateWidgetRequestData::new(
        CreateOrUpdateWidgetRequestAttributes::new(WidgetDefinition::new(
            "My Widget".to_string(),
            WidgetType::BAR_CHART,
        ))
        .tags(Some(vec![])),
        "widgets".to_string(),
    ));
    let configuration = datadog::Configuration::new();
    let api = WidgetsAPI::with_config(configuration);
    let resp = api
        .update_widget(
            WidgetExperienceType::CCM_REPORTS,
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
