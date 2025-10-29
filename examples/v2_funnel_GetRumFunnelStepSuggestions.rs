// Get rum funnel step suggestions returns "Successful response with funnel step
// suggestions" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_funnel::FunnelAPI;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequest;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestData;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestDataAttributes;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestDataAttributesSearch;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestDataAttributesSearchStepsItems;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestDataAttributesTermSearch;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestDataAttributesTime;
use datadog_api_client::datadogV2::model::FunnelSuggestionRequestDataType;

#[tokio::main]
async fn main() {
    let body = FunnelSuggestionRequest::new().data(
        FunnelSuggestionRequestData::new(
            FunnelSuggestionRequestDataType::FUNNEL_SUGGESTION_REQUEST,
        )
        .attributes(
            FunnelSuggestionRequestDataAttributes::new()
                .data_source("".to_string())
                .search(
                    FunnelSuggestionRequestDataAttributesSearch::new()
                        .cross_session_filter("".to_string())
                        .query_string("@type:view".to_string())
                        .steps(vec![
                            FunnelSuggestionRequestDataAttributesSearchStepsItems::new()
                                .facet("@view.name".to_string())
                                .step_filter("".to_string())
                                .value("/apm/home".to_string()),
                        ])
                        .subquery_id("".to_string()),
                )
                .term_search(
                    FunnelSuggestionRequestDataAttributesTermSearch::new().query("apm".to_string()),
                )
                .time(
                    FunnelSuggestionRequestDataAttributesTime::new()
                        .from(1756425600000)
                        .to(1756857600000),
                ),
        )
        .id("funnel_suggestion_request".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumFunnelStepSuggestions", true);
    let api = FunnelAPI::with_config(configuration);
    let resp = api.get_rum_funnel_step_suggestions(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
