// Create a new dashboard with query_table widget and text formatting
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::TableWidgetDefinition;
use datadog_api_client::datadogV1::model::TableWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TableWidgetHasSearchBar;
use datadog_api_client::datadogV1::model::TableWidgetRequest;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatMatch;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatMatchType;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatPalette;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatReplace;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatReplaceAll;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatReplaceAllType;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatRule;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(DashboardLayoutType::FREE, "Example-Dashboard".to_string(), vec![Widget::new(WidgetDefinition::TableWidgetDefinition(Box::new(TableWidgetDefinition::new(vec![TableWidgetRequest::new().formulas(vec![]).queries(vec![FormulaAndFunctionQueryDefinition::FormulaAndFunctionMetricQueryDefinition(Box::new(FormulaAndFunctionMetricQueryDefinition::new(FormulaAndFunctionMetricDataSource::METRICS, "query1".to_string(), "avg:aws.stream.globalaccelerator.processed_bytes_in{*} by {aws_account,acceleratoripaddress}".to_string(), ).aggregator(FormulaAndFunctionMetricAggregation::AVG))),FormulaAndFunctionQueryDefinition::FormulaAndFunctionMetricQueryDefinition(Box::new(FormulaAndFunctionMetricQueryDefinition::new(FormulaAndFunctionMetricDataSource::METRICS, "query2".to_string(), "avg:aws.stream.globalaccelerator.processed_bytes_out{*} by {aws_account,acceleratoripaddress}".to_string(), ).aggregator(FormulaAndFunctionMetricAggregation::AVG))),]).response_format(FormulaAndFunctionResponseFormat::SCALAR).text_formats(vec![vec![TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::IS, "fruit".to_string(), ), ).palette(TableWidgetTextFormatPalette::WHITE_ON_RED).replace(TableWidgetTextFormatReplace::TableWidgetTextFormatReplaceAll(Box::new(TableWidgetTextFormatReplaceAll::new(TableWidgetTextFormatReplaceAllType::ALL, "vegetable".to_string(), )))),TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::IS, "animal".to_string(), ), ).custom_bg_color("#632ca6".to_string()).palette(TableWidgetTextFormatPalette::CUSTOM_BG),TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::IS, "robot".to_string(), ), ).palette(TableWidgetTextFormatPalette::RED_ON_WHITE),TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::IS, "ai".to_string(), ), ).palette(TableWidgetTextFormatPalette::YELLOW_ON_WHITE),],vec![TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::IS_NOT, "xyz".to_string(), ), ).palette(TableWidgetTextFormatPalette::WHITE_ON_YELLOW),],vec![TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::CONTAINS, "test".to_string(), ), ).palette(TableWidgetTextFormatPalette::WHITE_ON_GREEN).replace(TableWidgetTextFormatReplace::TableWidgetTextFormatReplaceAll(Box::new(TableWidgetTextFormatReplaceAll::new(TableWidgetTextFormatReplaceAllType::ALL, "vegetable".to_string(), )))),],vec![TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::DOES_NOT_CONTAIN, "blah".to_string(), ), ).palette(TableWidgetTextFormatPalette::BLACK_ON_LIGHT_RED),],vec![TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::STARTS_WITH, "abc".to_string(), ), ).palette(TableWidgetTextFormatPalette::BLACK_ON_LIGHT_YELLOW),],vec![TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::ENDS_WITH, "xyz".to_string(), ), ).palette(TableWidgetTextFormatPalette::BLACK_ON_LIGHT_GREEN),TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::ENDS_WITH, "zzz".to_string(), ), ).palette(TableWidgetTextFormatPalette::GREEN_ON_WHITE),TableWidgetTextFormatRule::new(TableWidgetTextFormatMatch::new(TableWidgetTextFormatMatchType::IS, "animal".to_string(), ), ).custom_fg_color("#632ca6".to_string()).palette(TableWidgetTextFormatPalette::CUSTOM_TEXT),],]),], TableWidgetDefinitionType::QUERY_TABLE, ).has_search_bar(TableWidgetHasSearchBar::NEVER).title("".to_string()).title_align(WidgetTextAlign::LEFT).title_size("16".to_string()))), ).layout(WidgetLayout::new(4, 4, 0, 0, )),], ).description(Some("".to_string())).is_read_only(false).notify_list(Some(vec![])).template_variables(Some(vec![]));

    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
