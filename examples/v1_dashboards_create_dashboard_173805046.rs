// Create a new dashboard with slo widget
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "slo" in the system
    let slo_data_0_id = std::env::var("SLO_DATA_0_ID").unwrap();
    let body =
        Dashboard::new(
            DashboardLayoutType::FREE,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::SLOWidgetDefinition(
                        Box::new(
                            SLOWidgetDefinition::new(SLOWidgetDefinitionType::SLO, "detail".to_string())
                                .additional_query_filters("!host:excluded_host".to_string())
                                .global_time_target("0".to_string())
                                .show_error_budget(true)
                                .slo_id(slo_data_0_id)
                                .time_windows(vec![WidgetTimeWindows::SEVEN_DAYS])
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string())
                                .view_mode(WidgetViewMode::OVERALL),
                        ),
                    ),
                ).layout(WidgetLayout::new(21, 60, 0, 0))
            ],
        )
            .description(Some("".to_string()))
            .is_read_only(false)
            .notify_list(Some(vec![]))
            .template_variables(Some(vec![]));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
