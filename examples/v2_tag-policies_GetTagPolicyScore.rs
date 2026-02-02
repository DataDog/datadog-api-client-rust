// Get tag policy score returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_policies::TagPoliciesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetTagPolicyScore", true);
    let api = TagPoliciesAPI::with_config(configuration);
    let resp = api.get_tag_policy_score("123".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
