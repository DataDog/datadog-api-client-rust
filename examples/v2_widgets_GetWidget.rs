// Get a widget returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_widgets::WidgetsAPI;
use datadog_api_client::datadogV2::model::WidgetExperienceType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = WidgetsAPI::with_config(configuration);
    let resp = api
        .get_widget(
            WidgetExperienceType::CCM_REPORTS,
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
