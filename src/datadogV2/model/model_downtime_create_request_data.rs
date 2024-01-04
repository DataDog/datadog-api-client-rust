// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to create a downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeCreateRequestData {
    /// Downtime details.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::DowntimeCreateRequestAttributes>,
    /// Downtime resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DowntimeResourceType,
}

impl DowntimeCreateRequestData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::DowntimeCreateRequestAttributes>,
        type_: crate::datadogV2::model::DowntimeResourceType,
    ) -> DowntimeCreateRequestData {
        DowntimeCreateRequestData { attributes, type_ }
    }
}
