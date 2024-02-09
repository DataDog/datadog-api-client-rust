// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the API test configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsAPITestResultFullCheck {
    /// Configuration object for a Synthetic test.
    #[serde(rename = "config")]
    pub config: crate::datadogV1::model::SyntheticsTestConfig,
}

impl SyntheticsAPITestResultFullCheck {
    pub fn new(
        config: crate::datadogV1::model::SyntheticsTestConfig,
    ) -> SyntheticsAPITestResultFullCheck {
        SyntheticsAPITestResultFullCheck { config }
    }
}
