// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing the Synthetic tests to trigger.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerBody {
    /// Individual Synthetic test.
    #[serde(rename = "tests")]
    pub tests: Vec<crate::datadogV1::model::SyntheticsTriggerTest>,
}

impl SyntheticsTriggerBody {
    pub fn new(
        tests: Vec<crate::datadogV1::model::SyntheticsTriggerTest>,
    ) -> SyntheticsTriggerBody {
        SyntheticsTriggerBody { tests }
    }
}
