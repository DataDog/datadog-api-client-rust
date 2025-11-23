// Get rum cohort users returns "Successful response with cohort users" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cohort::CohortAPI;
use datadog_api_client::datadogV2::model::GetCohortUsersRequest;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestData;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributes;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributesDefinition;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFilters;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersAccountsItems;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersSegmentsItems;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersUsersItems;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataAttributesTime;
use datadog_api_client::datadogV2::model::GetCohortUsersRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        GetCohortUsersRequest
        ::new().data(
            GetCohortUsersRequestData::new(
                GetCohortUsersRequestDataType::COHORT_USERS_REQUEST,
            ).attributes(
                GetCohortUsersRequestDataAttributes::new()
                    .definition(
                        GetCohortUsersRequestDataAttributesDefinition
                        ::new().audience_filters(
                            GetCohortUsersRequestDataAttributesDefinitionAudienceFilters::new()
                                .accounts(
                                    vec![
                                        GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersAccountsItems::new(
                                            "".to_string(),
                                        )
                                    ],
                                )
                                .segments(
                                    vec![
                                        GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersSegmentsItems::new(
                                            "".to_string(),
                                            "".to_string(),
                                        )
                                    ],
                                )
                                .users(
                                    vec![
                                        GetCohortUsersRequestDataAttributesDefinitionAudienceFiltersUsersItems::new(
                                            "".to_string(),
                                        )
                                    ],
                                ),
                        ),
                    )
                    .time(GetCohortUsersRequestDataAttributesTime::new()),
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRumCohortUsers", true);
    let api = CohortAPI::with_config(configuration);
    let resp = api.get_rum_cohort_users(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
