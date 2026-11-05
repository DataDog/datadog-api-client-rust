// Create an email access policy returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateEmailAccessPolicyOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateStatusPageEmailAccessPolicyRequest;
use datadog_api_client::datadogV2::model::CreateStatusPageEmailAccessPolicyRequestData;
use datadog_api_client::datadogV2::model::CreateStatusPageEmailAccessPolicyRequestDataAttributes;
use datadog_api_client::datadogV2::model::ExecutionPolicyEffect;
use datadog_api_client::datadogV2::model::StatusPageEmailAccessPolicyType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateStatusPageEmailAccessPolicyRequest::new(
        CreateStatusPageEmailAccessPolicyRequestData::new(
            CreateStatusPageEmailAccessPolicyRequestDataAttributes::new(
                ExecutionPolicyEffect::ALLOW,
                "@example.com".to_string(),
            ),
            StatusPageEmailAccessPolicyType::EMAIL_ACCESS_POLICIES,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_email_access_policy(
            Uuid::parse_str("1234abcd-12ab-34cd-56ef-123456abcdef").expect("invalid UUID"),
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
