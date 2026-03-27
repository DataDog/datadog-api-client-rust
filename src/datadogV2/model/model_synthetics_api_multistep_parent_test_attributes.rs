// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a parent API multistep test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsApiMultistepParentTestAttributes {
    /// The name of the child subtest.
    #[serde(rename = "child_name")]
    pub child_name: Option<String>,
    /// The public ID of the child subtest.
    #[serde(rename = "child_public_id")]
    pub child_public_id: Option<String>,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id")]
    pub monitor_id: Option<i64>,
    /// Name of the parent test.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The overall state of the parent test.
    #[serde(rename = "overall_state")]
    pub overall_state: Option<i64>,
    /// Timestamp of when the overall state was last modified.
    #[serde(rename = "overall_state_modified")]
    pub overall_state_modified: Option<String>,
    /// The public ID of the parent test.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsApiMultistepParentTestAttributes {
    pub fn new() -> SyntheticsApiMultistepParentTestAttributes {
        SyntheticsApiMultistepParentTestAttributes {
            child_name: None,
            child_public_id: None,
            monitor_id: None,
            name: None,
            overall_state: None,
            overall_state_modified: None,
            public_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn child_name(mut self, value: String) -> Self {
        self.child_name = Some(value);
        self
    }

    pub fn child_public_id(mut self, value: String) -> Self {
        self.child_public_id = Some(value);
        self
    }

    pub fn monitor_id(mut self, value: i64) -> Self {
        self.monitor_id = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn overall_state(mut self, value: i64) -> Self {
        self.overall_state = Some(value);
        self
    }

    pub fn overall_state_modified(mut self, value: String) -> Self {
        self.overall_state_modified = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
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

impl Default for SyntheticsApiMultistepParentTestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsApiMultistepParentTestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsApiMultistepParentTestAttributesVisitor;
        impl<'a> Visitor<'a> for SyntheticsApiMultistepParentTestAttributesVisitor {
            type Value = SyntheticsApiMultistepParentTestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut child_name: Option<String> = None;
                let mut child_public_id: Option<String> = None;
                let mut monitor_id: Option<i64> = None;
                let mut name: Option<String> = None;
                let mut overall_state: Option<i64> = None;
                let mut overall_state_modified: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "child_name" => {
                            if v.is_null() {
                                continue;
                            }
                            child_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "child_public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            child_public_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_id" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overall_state" => {
                            if v.is_null() {
                                continue;
                            }
                            overall_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overall_state_modified" => {
                            if v.is_null() {
                                continue;
                            }
                            overall_state_modified =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsApiMultistepParentTestAttributes {
                    child_name,
                    child_public_id,
                    monitor_id,
                    name,
                    overall_state,
                    overall_state_modified,
                    public_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsApiMultistepParentTestAttributesVisitor)
    }
}
