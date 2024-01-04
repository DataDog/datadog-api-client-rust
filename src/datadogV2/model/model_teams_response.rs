// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with multiple teams
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsResponse {
    /// Teams response data
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::Team>>,
    /// Resources related to the team
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::TeamIncluded>>,
    /// Teams response links.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::TeamsResponseLinks>>,
    /// Teams response metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::TeamsResponseMeta>>,
}

impl TeamsResponse {
    pub fn new() -> TeamsResponse {
        TeamsResponse {
            data: None,
            included: None,
            links: None,
            meta: None,
        }
    }
}
