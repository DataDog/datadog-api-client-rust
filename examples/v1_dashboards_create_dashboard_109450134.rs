// Create a new dashboard with slo list widget with sort
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::FREE,
        "Example-Dashboard".to_string(),
        vec![
            Widget::new(WidgetDefinition::SLOListWidgetDefinition(Box::new(
                SLOListWidgetDefinition::new(
                    vec![SLOListWidgetRequest::new(
                        SLOListWidgetQuery::new("env:prod AND service:my-app".to_string())
                            .limit(75)
                            .sort(vec![WidgetFieldSort::new(
                                "status.sli".to_string(),
                                WidgetSort::ASCENDING,
                            )]),
                        SLOListWidgetRequestType::SLO_LIST,
                    )],
                    SLOListWidgetDefinitionType::SLO_LIST,
                )
                .title_align(WidgetTextAlign::LEFT)
                .title_size("16".to_string()),
            )))
            .layout(WidgetLayout::new(21, 60, 0, 0)),
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
