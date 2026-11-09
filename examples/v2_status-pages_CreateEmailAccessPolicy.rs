// Create an email access policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateEmailAccessPolicyOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::AccessPolicyEffect;
use datadog_api_client::datadogV2::model::CreateStatusPageEmailAccessPolicyRequest;
use datadog_api_client::datadogV2::model::CreateStatusPageEmailAccessPolicyRequestData;
use datadog_api_client::datadogV2::model::CreateStatusPageEmailAccessPolicyRequestDataAttributes;
use datadog_api_client::datadogV2::model::StatusPageEmailAccessPolicyType;

#[tokio::main]
async fn main() {
    // there is a valid "private_status_page" in the system
    let private_status_page_data_id =
        uuid::Uuid::parse_str(&std::env::var("PRIVATE_STATUS_PAGE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = CreateStatusPageEmailAccessPolicyRequest::new(
        CreateStatusPageEmailAccessPolicyRequestData::new(
            CreateStatusPageEmailAccessPolicyRequestDataAttributes::new(
                AccessPolicyEffect::ALLOW,
                "@example.com".to_string(),
            ),
            StatusPageEmailAccessPolicyType::EMAIL_ACCESS_POLICIES,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_email_access_policy(
            private_status_page_data_id.clone(),
            body,
            CreateEmailAccessPolicyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
