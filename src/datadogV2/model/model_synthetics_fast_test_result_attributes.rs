// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the fast test result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsFastTestResultAttributes {
    /// Device information for browser-based fast tests.
    #[serde(rename = "device")]
    pub device: Option<crate::datadogV2::model::SyntheticsFastTestResultDevice>,
    /// Location from which the fast test was executed.
    #[serde(rename = "location")]
    pub location: Option<crate::datadogV2::model::SyntheticsFastTestResultLocation>,
    /// Detailed result data for the fast test run. The exact shape of nested fields
    /// (`request`, `response`, `assertions`, etc.) depends on the test subtype.
    #[serde(rename = "result")]
    pub result: Option<crate::datadogV2::model::SyntheticsFastTestResultDetail>,
    /// Subtype of the Synthetic test that produced this result.
    #[serde(rename = "test_sub_type")]
    pub test_sub_type: Option<crate::datadogV2::model::SyntheticsFastTestSubType>,
    /// The type of the Synthetic test that produced this result (for example, `api` or `browser`).
    #[serde(rename = "test_type")]
    pub test_type: Option<String>,
    /// Version of the test at the time the fast test was triggered.
    #[serde(rename = "test_version")]
    pub test_version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsFastTestResultAttributes {
    pub fn new() -> SyntheticsFastTestResultAttributes {
        SyntheticsFastTestResultAttributes {
            device: None,
            location: None,
            result: None,
            test_sub_type: None,
            test_type: None,
            test_version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn device(
        mut self,
        value: crate::datadogV2::model::SyntheticsFastTestResultDevice,
    ) -> Self {
        self.device = Some(value);
        self
    }

    pub fn location(
        mut self,
        value: crate::datadogV2::model::SyntheticsFastTestResultLocation,
    ) -> Self {
        self.location = Some(value);
        self
    }

    pub fn result(
        mut self,
        value: crate::datadogV2::model::SyntheticsFastTestResultDetail,
    ) -> Self {
        self.result = Some(value);
        self
    }

    pub fn test_sub_type(
        mut self,
        value: crate::datadogV2::model::SyntheticsFastTestSubType,
    ) -> Self {
        self.test_sub_type = Some(value);
        self
    }

    pub fn test_type(mut self, value: String) -> Self {
        self.test_type = Some(value);
        self
    }

    pub fn test_version(mut self, value: i64) -> Self {
        self.test_version = Some(value);
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

impl Default for SyntheticsFastTestResultAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsFastTestResultAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsFastTestResultAttributesVisitor;
        impl<'a> Visitor<'a> for SyntheticsFastTestResultAttributesVisitor {
            type Value = SyntheticsFastTestResultAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut device: Option<crate::datadogV2::model::SyntheticsFastTestResultDevice> =
                    None;
                let mut location: Option<
                    crate::datadogV2::model::SyntheticsFastTestResultLocation,
                > = None;
                let mut result: Option<crate::datadogV2::model::SyntheticsFastTestResultDetail> =
                    None;
                let mut test_sub_type: Option<crate::datadogV2::model::SyntheticsFastTestSubType> =
                    None;
                let mut test_type: Option<String> = None;
                let mut test_version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "device" => {
                            if v.is_null() {
                                continue;
                            }
                            device = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Some(ref _test_sub_type) = test_sub_type {
                                match _test_sub_type {
                                    crate::datadogV2::model::SyntheticsFastTestSubType::UnparsedObject(_test_sub_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "test_type" => {
                            if v.is_null() {
                                continue;
                            }
                            test_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_version" => {
                            if v.is_null() {
                                continue;
                            }
                            test_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsFastTestResultAttributes {
                    device,
                    location,
                    result,
                    test_sub_type,
                    test_type,
                    test_version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsFastTestResultAttributesVisitor)
    }
}
