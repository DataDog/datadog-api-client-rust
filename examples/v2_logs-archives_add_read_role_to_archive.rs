// Grant role to an archive returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs_archives::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = RelationshipToRole::new().data(
        RelationshipToRoleData::new()
            .id("3653d3c6-0c75-11ea-ad28-fb5701eabc7d".to_string())
            .type_(RolesType::ROLES),
    );
    let configuration = Configuration::new();
    let api = LogsArchivesAPI::with_config(configuration);
    let resp = api
        .add_read_role_to_archive("archive_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}