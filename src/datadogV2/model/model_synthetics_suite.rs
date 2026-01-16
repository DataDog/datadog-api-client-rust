// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing details about a Synthetic suite.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsSuite {
    /// Notification message associated with the suite.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Name of the suite.
    #[serde(rename = "name")]
    pub name: String,
    /// Object describing the extra options for a Synthetic suite.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::SyntheticsSuiteOptions,
    /// The public ID for the test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Array of tags attached to the suite.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "tests")]
    pub tests: Vec<crate::datadogV2::model::SyntheticsSuiteTest>,
    /// Type of the Synthetic suite, `suite`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SyntheticsSuiteType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsSuite {
    pub fn new(
        name: String,
        options: crate::datadogV2::model::SyntheticsSuiteOptions,
        tests: Vec<crate::datadogV2::model::SyntheticsSuiteTest>,
        type_: crate::datadogV2::model::SyntheticsSuiteType,
    ) -> SyntheticsSuite {
        SyntheticsSuite {
            message: None,
            monitor_id: None,
            name,
            options,
            public_id: None,
            tags: None,
            tests,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }

    pub fn monitor_id(mut self, value: i64) -> Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsSuite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsSuiteVisitor;
        impl<'a> Visitor<'a> for SyntheticsSuiteVisitor {
            type Value = SyntheticsSuite;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut message: Option<String> = None;
                let mut monitor_id: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::SyntheticsSuiteOptions> = None;
                let mut public_id: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut tests: Option<Vec<crate::datadogV2::model::SyntheticsSuiteTest>> = None;
                let mut type_: Option<crate::datadogV2::model::SyntheticsSuiteType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tests" => {
                            tests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SyntheticsSuiteType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;
                let tests = tests.ok_or_else(|| M::Error::missing_field("tests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsSuite {
                    message,
                    monitor_id,
                    name,
                    options,
                    public_id,
                    tags,
                    tests,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsSuiteVisitor)
    }
}
