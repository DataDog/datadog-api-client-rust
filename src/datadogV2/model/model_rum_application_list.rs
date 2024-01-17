// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationList {
    /// RUM application list attributes.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::RUMApplicationListAttributes>,
    /// RUM application ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// RUM application list type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RUMApplicationListType,
}

impl RUMApplicationList {
    pub fn new(
        attributes: Box<crate::datadogV2::model::RUMApplicationListAttributes>,
        type_: crate::datadogV2::model::RUMApplicationListType,
    ) -> RUMApplicationList {
        RUMApplicationList {
            attributes,
            id: None,
            type_,
        }
    }
}
