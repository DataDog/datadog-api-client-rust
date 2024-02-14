// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team link create
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamLinkCreate {
    /// Team link attributes
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::TeamLinkAttributes,
    /// Team link type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamLinkType,
}

impl TeamLinkCreate {
    pub fn new(
        attributes: crate::datadogV2::model::TeamLinkAttributes,
        type_: crate::datadogV2::model::TeamLinkType,
    ) -> TeamLinkCreate {
        TeamLinkCreate { attributes, type_ }
    }
}
