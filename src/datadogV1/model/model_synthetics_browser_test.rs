// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing details about a Synthetic browser test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTest {
    /// Configuration object for a Synthetic browser test.
    #[serde(rename = "config")]
    pub config: crate::datadogV1::model::SyntheticsBrowserTestConfig,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Vec<String>,
    /// Notification message associated with the test. Message can either be text or an empty string.
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
    pub options: crate::datadogV1::model::SyntheticsTestOptions,
    /// The public ID of the test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Define whether you want to start (`live`) or pause (`paused`) a
    /// Synthetic test.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SyntheticsTestPauseStatus>,
    /// Array of steps for the test.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV1::model::SyntheticsStep>>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Type of the Synthetic test, `browser`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBrowserTestType,
}

impl SyntheticsBrowserTest {
    pub fn new(
        config: crate::datadogV1::model::SyntheticsBrowserTestConfig,
        locations: Vec<String>,
        message: String,
        name: String,
        options: crate::datadogV1::model::SyntheticsTestOptions,
        type_: crate::datadogV1::model::SyntheticsBrowserTestType,
    ) -> SyntheticsBrowserTest {
        SyntheticsBrowserTest {
            config,
            locations,
            message,
            monitor_id: None,
            name,
            options,
            public_id: None,
            status: None,
            steps: None,
            tags: None,
            type_,
        }
    }

    pub fn monitor_id(&mut self, value: i64) -> &mut Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }

    pub fn status(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestPauseStatus,
    ) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn steps(&mut self, value: Vec<crate::datadogV1::model::SyntheticsStep>) -> &mut Self {
        self.steps = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }
}
