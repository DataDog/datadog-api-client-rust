// Validate query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::RulesValidateQueryRequest;
use datadog_api_client::datadogV2::model::RulesValidateQueryRequestData;
use datadog_api_client::datadogV2::model::RulesValidateQueryRequestDataAttributes;
use datadog_api_client::datadogV2::model::RulesValidateQueryRequestDataType;

#[tokio::main]
async fn main() {
    let body = RulesValidateQueryRequest::new().data(
        RulesValidateQueryRequestData::new(RulesValidateQueryRequestDataType::VALIDATE_QUERY)
            .attributes(RulesValidateQueryRequestDataAttributes::new(
                "example:query AND test:true".to_string(),
            )),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.validate_query(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
