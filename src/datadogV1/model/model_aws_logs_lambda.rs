// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Description of the Lambdas.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsLambda {
    /// Available ARN IDs.
    #[serde(rename = "arn")]
    pub arn: Option<String>,
}

impl AWSLogsLambda {
    pub fn new() -> AWSLogsLambda {
        AWSLogsLambda { arn: None }
    }

    pub fn arn(&mut self, value: String) -> &mut Self {
        self.arn = Some(value);
        self
    }
}

impl Default for AWSLogsLambda {
    fn default() -> Self {
        Self::new()
    }
}
