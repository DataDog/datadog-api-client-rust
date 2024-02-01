// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for an AWS related account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSRelatedAccountAttributes {
    /// Whether or not the AWS account has a Datadog integration.
    #[serde(rename = "has_datadog_integration")]
    pub has_datadog_integration: Option<bool>,
    /// The name of the AWS account.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl AWSRelatedAccountAttributes {
    pub fn new() -> AWSRelatedAccountAttributes {
        AWSRelatedAccountAttributes {
            has_datadog_integration: None,
            name: None,
        }
    }

    pub fn has_datadog_integration(&mut self, value: bool) -> &mut Self {
        self.has_datadog_integration = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for AWSRelatedAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}
