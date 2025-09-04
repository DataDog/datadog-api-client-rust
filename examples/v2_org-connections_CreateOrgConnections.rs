// Create Org Connection returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_org_connections::OrgConnectionsAPI;
use datadog_api_client::datadogV2::model::OrgConnectionCreate;
use datadog_api_client::datadogV2::model::OrgConnectionCreateAttributes;
use datadog_api_client::datadogV2::model::OrgConnectionCreateRelationships;
use datadog_api_client::datadogV2::model::OrgConnectionCreateRequest;
use datadog_api_client::datadogV2::model::OrgConnectionOrgRelationship;
use datadog_api_client::datadogV2::model::OrgConnectionOrgRelationshipData;
use datadog_api_client::datadogV2::model::OrgConnectionOrgRelationshipDataType;
use datadog_api_client::datadogV2::model::OrgConnectionType;
use datadog_api_client::datadogV2::model::OrgConnectionTypeEnum;

#[tokio::main]
async fn main() {
    let body = OrgConnectionCreateRequest::new(OrgConnectionCreate::new(
        OrgConnectionCreateAttributes::new(vec![OrgConnectionTypeEnum::LOGS]),
        OrgConnectionCreateRelationships::new(
            OrgConnectionOrgRelationship::new().data(
                OrgConnectionOrgRelationshipData::new()
                    .id("83999dcd-7f97-11f0-8de1-1ecf66f1aa85".to_string())
                    .type_(OrgConnectionOrgRelationshipDataType::ORGS),
            ),
        ),
        OrgConnectionType::ORG_CONNECTION,
    ));
    let configuration = datadog::Configuration::new();
    let api = OrgConnectionsAPI::with_config(configuration);
    let resp = api.create_org_connections(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
