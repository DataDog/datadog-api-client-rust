// Get rum cohort returns "Successful response with cohort analysis data" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cohort::CohortAPI;
use datadog_api_client::datadogV2::model::GetCohortRequest;
use datadog_api_client::datadogV2::model::GetCohortRequestData;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributes;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributesDefinition;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFilters;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFiltersAccountsItems;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFiltersSegmentsItems;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributesDefinitionAudienceFiltersUsersItems;
use datadog_api_client::datadogV2::model::GetCohortRequestDataAttributesTime;
use datadog_api_client::datadogV2::model::GetCohortRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        GetCohortRequest
        ::new().data(
            GetCohortRequestData::new(
                GetCohortRequestDataType::COHORT_REQUEST,
            ).attributes(
                GetCohortRequestDataAttributes::new()
                    .definition(
                        GetCohortRequestDataAttributesDefinition
                        ::new().audience_filters(
                            GetCohortRequestDataAttributesDefinitionAudienceFilters::new()
                                .accounts(
                                    vec![
                                        GetCohortRequestDataAttributesDefinitionAudienceFiltersAccountsItems::new(
                                            "".to_string(),
                                        )
                                    ],
                                )
                                .segments(
                                    vec![
                                        GetCohortRequestDataAttributesDefinitionAudienceFiltersSegmentsItems::new(
                                            "".to_string(),
                                            "".to_string(),
                                        )
                                    ],
                                )
                                .users(
                                    vec![
                                        GetCohortRequestDataAttributesDefinitionAudienceFiltersUsersItems::new(
                                            "".to_string(),
                                        )
                                    ],
                                ),
                        ),
                    )
                    .time(GetCohortRequestDataAttributesTime::new()),
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumCohort", true);
    let api = CohortAPI::with_config(configuration);
    let resp = api.get_rum_cohort(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
