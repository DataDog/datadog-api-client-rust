// Update an Elastic Cloud CCM account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAccountUpdateData;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAccountUpdateRequest;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAuthentication;
use datadog_api_client::datadogV2::model::ElasticCloudCcmDataflow;
use datadog_api_client::datadogV2::model::ElasticCloudCcmDataflowId;
use datadog_api_client::datadogV2::model::ElasticCloudCcmSettingsUpdate;
use datadog_api_client::datadogV2::model::ElasticCloudCcmTokenAuth;
use datadog_api_client::datadogV2::model::ElasticCloudCcmTokenAuthType;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body = ElasticCloudCcmAccountUpdateRequest::new(ElasticCloudCcmAccountUpdateData::new(
        ElasticCloudCcmAccountUpdateAttributes::new()
            .authentication(ElasticCloudCcmAuthentication::ElasticCloudCcmTokenAuth(
                Box::new(ElasticCloudCcmTokenAuth::new(
                    "your-billing-api-key".to_string(),
                    ElasticCloudCcmTokenAuthType::BEARER_TOKEN,
                )),
            ))
            .dataflows(vec![ElasticCloudCcmDataflow::new(
                ElasticCloudCcmDataflowId::COST_DATA,
            )
            .enabled(true)])
            .name("elastic-cloud-ccm-prod".to_string())
            .settings(
                ElasticCloudCcmSettingsUpdate::new().elastic_org_id("2079364244".to_string()),
            ),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateElasticCloudCcmAccount", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .update_elastic_cloud_ccm_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
