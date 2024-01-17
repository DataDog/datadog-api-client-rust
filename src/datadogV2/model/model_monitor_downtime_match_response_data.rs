// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A downtime match.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorDowntimeMatchResponseData {
    /// Downtime match details.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::MonitorDowntimeMatchResponseAttributes>>,
    /// The downtime ID.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option")]
    pub id: Option<Option<String>>,
    /// Monitor Downtime Match resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MonitorDowntimeMatchResourceType>,
}

impl MonitorDowntimeMatchResponseData {
    pub fn new() -> MonitorDowntimeMatchResponseData {
        MonitorDowntimeMatchResponseData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for MonitorDowntimeMatchResponseData {
    fn default() -> Self {
        Self::new()
    }
}
