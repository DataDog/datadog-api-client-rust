// Create metric returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricNumeratorAttributes;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDataSourceType;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesDesiredChange;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation;
use datadog_api_client::datadogV2::model::ExperimentsDatadogMetricAggregationInput;
use datadog_api_client::datadogV2::model::ExperimentsDatadogMetricMeasureInput;
use datadog_api_client::datadogV2::model::MetricType;

#[tokio::main]
async fn main() {
    let body =
        ExperimentsCreateMetricV2Request::new(
            ExperimentsCreateMetricV2RequestData::new(
                ExperimentsCreateMetricV2RequestDataAttributes::ExperimentsCreateMetricNumeratorAttributes(
                    Box::new(
                        ExperimentsCreateMetricNumeratorAttributes::new(
                            ExperimentsCreateMetricV2RequestDataAttributesDataSourceType::DATADOG,
                            ExperimentsCreateMetricV2RequestDataAttributesDesiredChange::METRIC_INCREASES,
                            "ex-14bb9543f523edde".to_string(),
                            ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation
                            ::ExperimentsDatadogMetricAggregationInput(
                                Box::new(
                                    ExperimentsDatadogMetricAggregationInput::new(
                                        ExperimentsDatadogMetricMeasureInput::new(
                                            "double".to_string(),
                                            "ex-14bb9543f523edde view duration".to_string(),
                                            "RUM_VIEWS".to_string(),
                                            "PRODUCT_ANALYTICS".to_string(),
                                        ).column_name("@view.time_spent".to_string()),
                                        "sum".to_string(),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
                MetricType::METRICS,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_metric(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
