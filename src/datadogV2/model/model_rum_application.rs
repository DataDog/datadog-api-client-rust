// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplication {
    /// RUM application attributes.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::RUMApplicationAttributes,
    /// RUM application ID.
    #[serde(rename = "id")]
    pub id: String,
    /// RUM application response type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RUMApplicationType,
}

impl RUMApplication {
    pub fn new(
        attributes: crate::datadogV2::model::RUMApplicationAttributes,
        id: String,
        type_: crate::datadogV2::model::RUMApplicationType,
    ) -> RUMApplication {
        RUMApplication {
            attributes,
            id,
            type_,
        }
    }
}
