// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team link
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamLink {
    /// Team link attributes
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::TeamLinkAttributes>,
    /// The team link's identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Team link type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamLinkType,
}

impl TeamLink {
    pub fn new(
        attributes: Box<crate::datadogV2::model::TeamLinkAttributes>,
        id: String,
        type_: crate::datadogV2::model::TeamLinkType,
    ) -> TeamLink {
        TeamLink {
            attributes,
            id,
            type_,
        }
    }
}
