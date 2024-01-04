// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing details about your Synthetic test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestDetails {
    /// Configuration object for a Synthetic test.
    #[serde(rename = "config")]
    pub config: Option<Box<crate::datadogV1::model::SyntheticsTestConfig>>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<Box<crate::datadogV1::model::Creator>>,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<String>>,
    /// Notification message associated with the test.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Name of the test.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Object describing the extra options for a Synthetic test.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV1::model::SyntheticsTestOptions>>,
    /// The test public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    /// For browser test, the steps of the test.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV1::model::SyntheticsStep>>,
    /// The subtype of the Synthetic API test, `http`, `ssl`, `tcp`,
    /// `dns`, `icmp`, `udp`, `websocket`, `grpc` or `multi`.
    #[serde(rename = "subtype")]
    pub subtype: Option<crate::datadogV1::model::SyntheticsTestDetailsSubType>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the Synthetic test, either `api` or `browser`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsTestDetailsType>,
}

impl SyntheticsTestDetails {
    pub fn new() -> SyntheticsTestDetails {
        SyntheticsTestDetails {
            config: None,
            creator: None,
            locations: None,
            message: None,
            monitor_id: None,
            name: None,
            options: None,
            public_id: None,
            status: None,
            steps: None,
            subtype: None,
            tags: None,
            type_: None,
        }
    }
}
