// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the Sensitive Data Scanner standard pattern.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerStandardPatternAttributes {
    /// Description of the standard pattern.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// List of included keywords.
    #[serde(rename = "included_keywords")]
    pub included_keywords: Option<Vec<String>>,
    /// Name of the standard pattern.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Regex to match.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Integer from 1 (high) to 5 (low) indicating standard pattern issue severity.
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// List of tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}

impl SensitiveDataScannerStandardPatternAttributes {
    pub fn new() -> SensitiveDataScannerStandardPatternAttributes {
        SensitiveDataScannerStandardPatternAttributes {
            description: None,
            included_keywords: None,
            name: None,
            pattern: None,
            priority: None,
            tags: None,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn included_keywords(mut self, value: Vec<String>) -> Self {
        self.included_keywords = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    pub fn priority(mut self, value: i64) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerStandardPatternAttributes {
    fn default() -> Self {
        Self::new()
    }
}
