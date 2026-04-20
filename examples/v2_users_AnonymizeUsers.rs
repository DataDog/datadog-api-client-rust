// Anonymize users returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_users::UsersAPI;
use datadog_api_client::datadogV2::model::AnonymizeUsersRequest;
use datadog_api_client::datadogV2::model::AnonymizeUsersRequestAttributes;
use datadog_api_client::datadogV2::model::AnonymizeUsersRequestData;
use datadog_api_client::datadogV2::model::AnonymizeUsersRequestType;

#[tokio::main]
async fn main() {
    let body = AnonymizeUsersRequest::new(
        AnonymizeUsersRequestData::new(
            AnonymizeUsersRequestAttributes::new(vec![
                "00000000-0000-0000-0000-000000000000".to_string()
            ]),
            AnonymizeUsersRequestType::ANONYMIZE_USERS_REQUEST,
        )
        .id("00000000-0000-0000-0000-000000000000".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AnonymizeUsers", true);
    let api = UsersAPI::with_config(configuration);
    let resp = api.anonymize_users(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
