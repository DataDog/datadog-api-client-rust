// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInvitationResponseData {
    /// Attributes of a user invitation.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: UserInvitationDataAttributes,
    /// ID of the user invitation.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: String,
    /// Relationships data for user invitation.
    #[serde(rename = "relationships")]
    pub relationships: UserInvitationRelationships,
    /// User invitations type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: UserInvitationsType,
}

