// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToUserTeamUser {
    /// A user's relationship with a team
    #[serde(rename = "data")]
    pub data: RelationshipToUserTeamUserData,
}

