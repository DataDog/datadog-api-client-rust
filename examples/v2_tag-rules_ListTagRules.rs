// List tag rules returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_tag_rules::ListTagRulesOptionalParams;
use datadog_api_client::datadogV2::api_tag_rules::TagRulesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListTagRules", true);
    let api = TagRulesAPI::with_config(configuration);
    let resp = api
        .list_tag_rules(ListTagRulesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
