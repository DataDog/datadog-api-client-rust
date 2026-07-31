// Create an Elastic Cloud CCM account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAccountAttributes;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAccountCreateData;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAccountRequest;
use datadog_api_client::datadogV2::model::ElasticCloudCcmAuthentication;
use datadog_api_client::datadogV2::model::ElasticCloudCcmDataflow;
use datadog_api_client::datadogV2::model::ElasticCloudCcmDataflowId;
use datadog_api_client::datadogV2::model::ElasticCloudCcmSettings;
use datadog_api_client::datadogV2::model::ElasticCloudCcmTokenAuth;
use datadog_api_client::datadogV2::model::ElasticCloudCcmTokenAuthType;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body = ElasticCloudCcmAccountRequest::new(ElasticCloudCcmAccountCreateData::new(
        ElasticCloudCcmAccountAttributes::new(
            ElasticCloudCcmAuthentication::ElasticCloudCcmTokenAuth(Box::new(
                ElasticCloudCcmTokenAuth::new(
                    "your-billing-api-key".to_string(),
                    ElasticCloudCcmTokenAuthType::BEARER_TOKEN,
                ),
            )),
            "elastic-cloud-ccm-prod".to_string(),
        )
        .dataflows(vec![ElasticCloudCcmDataflow::new(
            ElasticCloudCcmDataflowId::COST_DATA,
        )
        .enabled(true)])
        .settings(ElasticCloudCcmSettings::new("2079364244".to_string())),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateElasticCloudCcmAccount", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api.create_elastic_cloud_ccm_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
