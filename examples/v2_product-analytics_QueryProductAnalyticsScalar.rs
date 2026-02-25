// Compute scalar analytics returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsBaseQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsCompute;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQueryDataSource;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventSearch;

#[tokio::main]
async fn main() {
    let body = ProductAnalyticsAnalyticsRequest::new(ProductAnalyticsAnalyticsRequestData::new(
        ProductAnalyticsAnalyticsRequestAttributes::new(
            1771232048460,
            ProductAnalyticsAnalyticsQuery::new(
                ProductAnalyticsCompute::new("count".to_string()),
                ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(Box::new(
                    ProductAnalyticsEventQuery::new(
                        ProductAnalyticsEventQueryDataSource::PRODUCT_ANALYTICS,
                        ProductAnalyticsEventSearch::new().query("@type:view".to_string()),
                    ),
                )),
            ),
            1771836848262,
        ),
        ProductAnalyticsAnalyticsRequestType::FORMULA_ANALYTICS_EXTENDED_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_scalar(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
