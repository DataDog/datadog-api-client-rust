// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data for the user who created the downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeRelationshipsCreatedByData {
    /// User ID of the downtime creator.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Users resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::UsersType>,
}

impl DowntimeRelationshipsCreatedByData {
    pub fn new() -> DowntimeRelationshipsCreatedByData {
        DowntimeRelationshipsCreatedByData {
            id: None,
            type_: None,
        }
    }
}
