// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to update a downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeUpdateRequestData {
    /// Attributes of the downtime to update.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::DowntimeUpdateRequestAttributes,
    /// ID of this downtime.
    #[serde(rename = "id")]
    pub id: String,
    /// Downtime resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DowntimeResourceType,
}

impl DowntimeUpdateRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::DowntimeUpdateRequestAttributes,
        id: String,
        type_: crate::datadogV2::model::DowntimeResourceType,
    ) -> DowntimeUpdateRequestData {
        DowntimeUpdateRequestData {
            attributes,
            id,
            type_,
        }
    }
}
