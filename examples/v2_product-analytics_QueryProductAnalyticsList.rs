// List analytics events returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsListQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsListRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsListRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsListRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsAnalyticsListRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsBaseQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQueryDataSource;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventSearch;

#[tokio::main]
async fn main() {
    let body =
        ProductAnalyticsAnalyticsListRequest::new(ProductAnalyticsAnalyticsListRequestData::new(
            ProductAnalyticsAnalyticsListRequestAttributes::new(
                1771232048460,
                ProductAnalyticsAnalyticsListQuery::new(
                    ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(Box::new(
                        ProductAnalyticsEventQuery::new(
                            ProductAnalyticsEventQueryDataSource::PRODUCT_ANALYTICS,
                            ProductAnalyticsEventSearch::new().query("@type:view".to_string()),
                        ),
                    )),
                )
                .columns(vec!["@view.name".to_string()])
                .limit(100),
                1771836848262,
            ),
            ProductAnalyticsAnalyticsListRequestType::FORMULA_ANALYTICS_EXTENDED_LIST_REQUEST,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryProductAnalyticsList", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_list(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
