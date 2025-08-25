// Delete case comment returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;

#[tokio::main]
async fn main() {
    // there is a valid "case" in the system
    let case_id = std::env::var("CASE_ID").unwrap();

    // there is a valid "comment" in the system
    let comment_id = std::env::var("COMMENT_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .delete_case_comment(case_id.clone(), comment_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
