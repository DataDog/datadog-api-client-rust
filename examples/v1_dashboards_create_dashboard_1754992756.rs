// Create a new dashboard with powerpack widget
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "powerpack" in the system
    let powerpack_data_id = std::env::var("POWERPACK_DATA_ID").unwrap();
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard with powerpack widget".to_string(),
        vec![
            Widget::new(WidgetDefinition::PowerpackWidgetDefinition(Box::new(
                PowerpackWidgetDefinition::new(
                    powerpack_data_id.clone(),
                    PowerpackWidgetDefinitionType::POWERPACK,
                )
                .template_variables(
                    PowerpackTemplateVariables::new()
                        .controlled_by_powerpack(vec![PowerpackTemplateVariableContents::new(
                            "foo".to_string(),
                            vec!["baz".to_string(), "qux".to_string(), "quuz".to_string()],
                        )
                        .prefix("bar".to_string())])
                        .controlled_externally(vec![]),
                ),
            )))
            .layout(WidgetLayout::new(2, 2, 1, 1).is_column_break(false)),
        ],
    )
    .description(Some("description".to_string()))
    .is_read_only(false);
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
