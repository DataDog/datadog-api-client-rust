// Get user facet info returns "Successful response with facet information"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::FacetInfoRequest;
use datadog_api_client::datadogV2::model::FacetInfoRequestData;
use datadog_api_client::datadogV2::model::FacetInfoRequestDataAttributes;
use datadog_api_client::datadogV2::model::FacetInfoRequestDataAttributesSearch;
use datadog_api_client::datadogV2::model::FacetInfoRequestDataAttributesTermSearch;
use datadog_api_client::datadogV2::model::FacetInfoRequestDataType;

#[tokio::main]
async fn main() {
    let body = FacetInfoRequest::new().data(
        FacetInfoRequestData::new(FacetInfoRequestDataType::USERS_FACET_INFO_REQUEST)
            .attributes(
                FacetInfoRequestDataAttributes::new("first_browser_name".to_string(), 10)
                    .search(
                        FacetInfoRequestDataAttributesSearch::new()
                            .query("user_org_id:5001 AND first_country_code:US".to_string()),
                    )
                    .term_search(
                        FacetInfoRequestDataAttributesTermSearch::new().value("Chrome".to_string()),
                    ),
            )
            .id("facet_info_request".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetUserFacetInfo", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.get_user_facet_info(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
