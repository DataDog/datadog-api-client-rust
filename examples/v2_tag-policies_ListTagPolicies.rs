// List tag policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_policies::ListTagPoliciesOptionalParams;
use datadog_api_client::datadogV2::api_tag_policies::TagPoliciesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListTagPolicies", true);
    let api = TagPoliciesAPI::with_config(configuration);
    let resp = api
        .list_tag_policies(ListTagPoliciesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
