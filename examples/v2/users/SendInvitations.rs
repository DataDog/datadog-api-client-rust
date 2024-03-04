// Send invitation emails returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_users::UsersAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "user" in the system
    let user_data_id = std::env::var("USER_DATA_ID").unwrap();
    let body =
        UserInvitationsRequest::new(
            vec![
                UserInvitationData::new(
                    UserInvitationRelationships::new(
                        RelationshipToUser::new(RelationshipToUserData::new(user_data_id, UsersType::USERS)),
                    ),
                    UserInvitationsType::USER_INVITATIONS,
                )
            ],
        );
    let configuration = Configuration::new();
    let api = UsersAPI::with_config(configuration);
    let resp = api.send_invitations(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
