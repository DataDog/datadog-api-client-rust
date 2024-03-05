// Get all teams with fields_team parameter returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_teams::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = TeamsAPI::with_config(configuration);
    let resp =
        api
            .list_teams(
                ListTeamsOptionalParams
                ::default().fields_team(vec![TeamsField::ID, TeamsField::NAME, TeamsField::HANDLE]),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
