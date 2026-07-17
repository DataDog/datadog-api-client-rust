// Create a new dashboard with a live default_timeframe returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardDefaultTimeframeSetting;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DashboardLiveTimeframe;
use datadog_api_client::datadogV1::model::DashboardLiveTimeframeType;
use datadog_api_client::datadogV1::model::NoteWidgetDefinition;
use datadog_api_client::datadogV1::model::NoteWidgetDefinitionType;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLiveSpanUnit;
use datadog_api_client::datadogV1::model::WidgetTextAlign;
use datadog_api_client::datadogV1::model::WidgetTickEdge;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard".to_string(),
        vec![Widget::new(WidgetDefinition::NoteWidgetDefinition(
            Box::new(
                NoteWidgetDefinition::new("test".to_string(), NoteWidgetDefinitionType::NOTE)
                    .background_color("white".to_string())
                    .font_size("14".to_string())
                    .show_tick(false)
                    .text_align(WidgetTextAlign::LEFT)
                    .tick_edge(WidgetTickEdge::LEFT)
                    .tick_pos("50%".to_string()),
            ),
        ))],
    )
    .default_timeframe(DashboardDefaultTimeframeSetting::DashboardLiveTimeframe(
        Box::new(DashboardLiveTimeframe::new(
            DashboardLiveTimeframeType::LIVE,
            WidgetLiveSpanUnit::HOUR,
            4,
        )),
    ));
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
