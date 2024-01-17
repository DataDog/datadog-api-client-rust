// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object which includes all powerpack configurations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPowerpacksResponse {
    /// List of powerpack definitions.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::PowerpackData>>,
    /// Array of objects related to the users.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::User>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::PowerpackResponseLinks>>,
    /// Powerpack response metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::PowerpacksResponseMeta>>,
}

impl ListPowerpacksResponse {
    pub fn new() -> ListPowerpacksResponse {
        ListPowerpacksResponse {
            data: None,
            included: None,
            links: None,
            meta: None,
        }
    }
}
