// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes of the Sensitive Data Scanner rule.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerRuleAttributes {
    /// Description of the rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Attributes excluded from the scan. If namespaces is provided, it has to be a sub-path of the namespaces array.
    #[serde(rename = "excluded_namespaces")]
    pub excluded_namespaces: Option<Vec<String>>,
    /// Whether or not the rule is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Attributes included in the scan. If namespaces is empty or missing, all attributes except excluded_namespaces are scanned.
    /// If both are missing the whole event is scanned.
    #[serde(rename = "namespaces")]
    pub namespaces: Option<Vec<String>>,
    /// Not included if there is a relationship to a standard pattern.
    #[serde(rename = "pattern")]
    pub pattern: Option<String>,
    /// Integer from 1 (high) to 5 (low) indicating rule issue severity.
    #[serde(rename = "priority")]
    pub priority: Option<i64>,
    /// List of tags.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Object describing how the scanned event will be replaced.
    #[serde(rename = "text_replacement")]
    pub text_replacement: Option<Box<crate::datadogV2::model::SensitiveDataScannerTextReplacement>>,
}

impl SensitiveDataScannerRuleAttributes {
    pub fn new() -> SensitiveDataScannerRuleAttributes {
        SensitiveDataScannerRuleAttributes {
            description: None,
            excluded_namespaces: None,
            is_enabled: None,
            name: None,
            namespaces: None,
            pattern: None,
            priority: None,
            tags: None,
            text_replacement: None,
        }
    }
}