// Create a new dashboard with heatmap widget with markers and num_buckets
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::HeatMapWidgetDefinition;
use datadog_api_client::datadogV1::model::HeatMapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::HeatMapWidgetRequest;
use datadog_api_client::datadogV1::model::HeatMapWidgetXAxis;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetAxis;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetHistogramRequestType;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetMarker;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard".to_string(),
        vec![
            Widget::new(WidgetDefinition::HeatMapWidgetDefinition(Box::new(
                HeatMapWidgetDefinition::new(
                    vec![HeatMapWidgetRequest::new()
                        .query(FormulaAndFunctionMetricQueryDefinition::new(
                            FormulaAndFunctionMetricDataSource::METRICS,
                            "query1".to_string(),
                            "histogram:trace.servlet.request{*}".to_string(),
                        ))
                        .request_type(WidgetHistogramRequestType::HISTOGRAM)],
                    HeatMapWidgetDefinitionType::HEATMAP,
                )
                .markers(vec![
                    WidgetMarker::new("50".to_string()).display_type("percentile".to_string()),
                    WidgetMarker::new("99".to_string()).display_type("percentile".to_string()),
                ])
                .title("".to_string())
                .title_align(WidgetTextAlign::LEFT)
                .title_size("16".to_string())
                .xaxis(HeatMapWidgetXAxis::new().num_buckets(75))
                .yaxis(
                    WidgetAxis::new()
                        .include_zero(true)
                        .max("auto".to_string())
                        .min("auto".to_string())
                        .scale("linear".to_string()),
                ),
            )))
            .layout(WidgetLayout::new(4, 4, 0, 0)),
        ],
    );
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
