// Get rum sankey returns "Successful response with Sankey diagram data" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_user_flow::UserFlowAPI;
use datadog_api_client::datadogV2::model::SankeyRequest;
use datadog_api_client::datadogV2::model::SankeyRequestData;
use datadog_api_client::datadogV2::model::SankeyRequestDataAttributes;
use datadog_api_client::datadogV2::model::SankeyRequestDataAttributesDefinition;
use datadog_api_client::datadogV2::model::SankeyRequestDataAttributesSampling;
use datadog_api_client::datadogV2::model::SankeyRequestDataAttributesSearch;
use datadog_api_client::datadogV2::model::SankeyRequestDataAttributesSearchAudienceFilters;
use datadog_api_client::datadogV2::model::SankeyRequestDataAttributesTime;
use datadog_api_client::datadogV2::model::SankeyRequestDataType;

#[tokio::main]
async fn main() {
    let body = SankeyRequest::new().data(
        SankeyRequestData::new(SankeyRequestDataType::SANKEY_REQUEST)
            .attributes(
                SankeyRequestDataAttributes::new()
                    .data_source("".to_string())
                    .definition(
                        SankeyRequestDataAttributesDefinition::new()
                            .entries_per_step(10)
                            .number_of_steps(5)
                            .source("@view.name".to_string())
                            .target("@view.name".to_string()),
                    )
                    .enforced_execution_type("".to_string())
                    .request_id("".to_string())
                    .sampling(SankeyRequestDataAttributesSampling::new().enabled(true))
                    .search(
                        SankeyRequestDataAttributesSearch::new()
                            .audience_filters(
                                SankeyRequestDataAttributesSearchAudienceFilters::new(),
                            )
                            .query("@type:view @application.id:*".to_string())
                            .subquery_id("".to_string()),
                    )
                    .time(
                        SankeyRequestDataAttributesTime::new()
                            .from(1756425600000)
                            .to(1756857600000),
                    ),
            )
            .id("sankey_request".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumSankey", true);
    let api = UserFlowAPI::with_config(configuration);
    let resp = api.get_rum_sankey(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
