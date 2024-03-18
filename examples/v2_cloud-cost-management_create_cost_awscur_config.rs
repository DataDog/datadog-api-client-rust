// Create Cloud Cost Management AWS CUR config returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_cloud_cost_management::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = AwsCURConfigPostRequest::new(AwsCURConfigPostData::new(
        AwsCURConfigPostRequestAttributes::new(
            "123456789123".to_string(),
            "dd-cost-bucket".to_string(),
            "dd-report-name".to_string(),
            "dd-report-prefix".to_string(),
        )
        .bucket_region("us-east-1".to_string()),
        AwsCURConfigPostRequestType::AWS_CUR_CONFIG_POST_REQUEST,
    ));
    let configuration = Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.create_cost_awscur_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
