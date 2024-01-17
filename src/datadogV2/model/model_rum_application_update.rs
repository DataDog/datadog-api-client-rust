// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application update.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationUpdate {
    /// RUM application update attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::RUMApplicationUpdateAttributes>>,
    /// RUM application ID.
    #[serde(rename = "id")]
    pub id: String,
    /// RUM application update type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RUMApplicationUpdateType,
}

impl RUMApplicationUpdate {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::RUMApplicationUpdateType,
    ) -> RUMApplicationUpdate {
        RUMApplicationUpdate {
            attributes: None,
            id,
            type_,
        }
    }
}
