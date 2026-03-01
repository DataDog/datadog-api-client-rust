// Register a token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_prem_management_service::OnPremManagementServiceAPI;
use datadog_api_client::datadogV2::model::OnPremManagementServiceRegisterTokenAttributes;
use datadog_api_client::datadogV2::model::OnPremManagementServiceRegisterTokenDataRequest;
use datadog_api_client::datadogV2::model::OnPremManagementServiceRegisterTokenRequest;
use datadog_api_client::datadogV2::model::OnPremManagementServiceRegisterTokenType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = OnPremManagementServiceRegisterTokenRequest::new(
        OnPremManagementServiceRegisterTokenDataRequest::new(
            OnPremManagementServiceRegisterTokenAttributes::new(
                Uuid::parse_str("2f66ec56-d1f3-4a18-908d-5e8c12dfb3b0").expect("invalid UUID"),
                "runner-GBUyh2Tm6oKS6ap4kt8Bbx".to_string(),
            )
            .app_id(Uuid::parse_str("9a93d314-ca37-461d-b18e-0587f03c6e2a").expect("invalid UUID"))
            .app_version(5)
            .query_id(
                Uuid::parse_str("8744d459-18aa-405b-821e-df9bb101c01e").expect("invalid UUID"),
            ),
            OnPremManagementServiceRegisterTokenType::REGISTERTOKENREQUEST,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.RegisterOnPremManagementServiceToken", true);
    let api = OnPremManagementServiceAPI::with_config(configuration);
    let resp = api.register_on_prem_management_service_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
