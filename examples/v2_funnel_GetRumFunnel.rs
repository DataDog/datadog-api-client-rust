// Get rum funnel returns "Successful response with funnel analysis data" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_funnel::FunnelAPI;
use datadog_api_client::datadogV2::model::FunnelRequest;
use datadog_api_client::datadogV2::model::FunnelRequestData;
use datadog_api_client::datadogV2::model::FunnelRequestDataAttributes;
use datadog_api_client::datadogV2::model::FunnelRequestDataAttributesSearch;
use datadog_api_client::datadogV2::model::FunnelRequestDataAttributesSearchStepsItems;
use datadog_api_client::datadogV2::model::FunnelRequestDataAttributesTime;
use datadog_api_client::datadogV2::model::FunnelRequestDataType;

#[tokio::main]
async fn main() {
    let body = FunnelRequest::new().data(
        FunnelRequestData::new(FunnelRequestDataType::FUNNEL_REQUEST)
            .attributes(
                FunnelRequestDataAttributes::new()
                    .data_source("rum".to_string())
                    .enforced_execution_type("".to_string())
                    .request_id("".to_string())
                    .search(
                        FunnelRequestDataAttributesSearch::new()
                            .cross_session_filter("".to_string())
                            .query_string("@type:view".to_string())
                            .steps(vec![
                                FunnelRequestDataAttributesSearchStepsItems::new()
                                    .facet("@view.name".to_string())
                                    .step_filter("".to_string())
                                    .value("/apm/home".to_string()),
                                FunnelRequestDataAttributesSearchStepsItems::new()
                                    .facet("@view.name".to_string())
                                    .step_filter("".to_string())
                                    .value("/apm/traces".to_string()),
                            ])
                            .subquery_id("".to_string()),
                    )
                    .time(
                        FunnelRequestDataAttributesTime::new()
                            .from(1756425600000)
                            .to(1756857600000),
                    ),
            )
            .id("funnel_request".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumFunnel", true);
    let api = FunnelAPI::with_config(configuration);
    let resp = api.get_rum_funnel(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
