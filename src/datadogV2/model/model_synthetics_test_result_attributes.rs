// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a Synthetic test result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultAttributes {
    /// Batch information for the test result.
    #[serde(rename = "batch")]
    pub batch: Option<crate::datadogV2::model::SyntheticsTestResultBatch>,
    /// CI information associated with the test result.
    #[serde(rename = "ci")]
    pub ci: Option<crate::datadogV2::model::SyntheticsTestResultCI>,
    /// Device information for the test result (browser and mobile tests).
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV2::model::SyntheticsTestResultDevice>,
    /// Git information associated with the test result.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV2::model::SyntheticsTestResultGit>,
    /// Location information for a Synthetic test result.
    #[serde(rename = "location")]
    pub location: Option<crate::datadogV2::model::SyntheticsTestResultLocation>,
    /// Full result details for a Synthetic test execution.
    #[serde(rename = "result")]
    pub result: Option<crate::datadogV2::model::SyntheticsTestResultDetail>,
    /// The subtype of the test (for example, `http`, `multi`, `ssl`).
    #[serde(rename = "test_sub_type")]
    pub test_sub_type: Option<String>,
    /// The type of the test (for example, `api` or `browser`).
    #[serde(rename = "test_type")]
    pub test_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultAttributes {
    pub fn new() -> SyntheticsTestResultAttributes {
        SyntheticsTestResultAttributes {
            batch: None,
            ci: None,
            device: None,
            git: None,
            location: None,
            result: None,
            test_sub_type: None,
            test_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn batch(mut self, value: crate::datadogV2::model::SyntheticsTestResultBatch) -> Self {
        self.batch = Some(value);
        self
    }

    pub fn ci(mut self, value: crate::datadogV2::model::SyntheticsTestResultCI) -> Self {
        self.ci = Some(value);
        self
    }

    pub fn device(mut self, value: crate::datadogV2::model::SyntheticsTestResultDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn git(mut self, value: crate::datadogV2::model::SyntheticsTestResultGit) -> Self {
        self.git = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn result(mut self, value: crate::datadogV2::model::SyntheticsTestResultDetail) -> Self {
        self.result = Some(value);
        self
    }

    pub fn test_sub_type(mut self, value: String) -> Self {
        self.test_sub_type = Some(value);
        self
    }

    pub fn test_type(mut self, value: String) -> Self {
        self.test_type = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestResultAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultAttributesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultAttributesVisitor {
            type Value = SyntheticsTestResultAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut batch: Option<crate::datadogV2::model::SyntheticsTestResultBatch> = None;
                let mut ci: Option<crate::datadogV2::model::SyntheticsTestResultCI> = None;
                let mut device: Option<crate::datadogV2::model::SyntheticsTestResultDevice> = None;
                let mut git: Option<crate::datadogV2::model::SyntheticsTestResultGit> = None;
                let mut location: Option<crate::datadogV2::model::SyntheticsTestResultLocation> =
                    None;
                let mut result: Option<crate::datadogV2::model::SyntheticsTestResultDetail> = None;
                let mut test_sub_type: Option<String> = None;
                let mut test_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "batch" => {
                            if v.is_null() {
                                continue;
                            }
                            batch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci" => {
                            if v.is_null() {
                                continue;
                            }
                            ci = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "device" => {
                            if v.is_null() {
                                continue;
                            }
                            device = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git" => {
                            if v.is_null() {
                                continue;
                            }
                            git = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result" => {
                            if v.is_null() {
                                continue;
                            }
                            result = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_sub_type" => {
                            if v.is_null() {
                                continue;
                            }
                            test_sub_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_type" => {
                            if v.is_null() {
                                continue;
                            }
                            test_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultAttributes {
                    batch,
                    ci,
                    device,
                    git,
                    location,
                    result,
                    test_sub_type,
                    test_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultAttributesVisitor)
    }
}
