// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing details about a Synthetic API test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITest {
    /// Configuration object for a Synthetic API test.
    #[serde(rename = "config")]
    pub config: Box<crate::datadogV1::model::SyntheticsAPITestConfig>,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Vec<String>,
    /// Notification message associated with the test.
    #[serde(rename = "message")]
    pub message: String,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Name of the test.
    #[serde(rename = "name")]
    pub name: String,
    /// Object describing the extra options for a Synthetic test.
    #[serde(rename = "options")]
    pub options: Box<crate::datadogV1::model::SyntheticsTestOptions>,
    /// The public ID for the test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    /// The subtype of the Synthetic API test, `http`, `ssl`, `tcp`,
    /// `dns`, `icmp`, `udp`, `websocket`, `grpc` or `multi`.
    #[serde(rename = "subtype")]
    pub subtype: Option<crate::datadogV1::model::SyntheticsTestDetailsSubType>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the Synthetic test, `api`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsAPITestType,
}

impl SyntheticsAPITest {
    pub fn new(
        config: Box<crate::datadogV1::model::SyntheticsAPITestConfig>,
        locations: Vec<String>,
        message: String,
        name: String,
        options: Box<crate::datadogV1::model::SyntheticsTestOptions>,
        type_: crate::datadogV1::model::SyntheticsAPITestType,
    ) -> SyntheticsAPITest {
        SyntheticsAPITest {
            config,
            locations,
            message,
            monitor_id: None,
            name,
            options,
            public_id: None,
            status: None,
            subtype: None,
            tags: None,
            type_,
        }
    }
}
