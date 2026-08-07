// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance control that can be updated. Only the attributes present in the request are modified.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceControlUpdateAttributes {
    /// A free-form map of parameter names to their configured values.
    #[serde(rename = "detection_parameters")]
    pub detection_parameters: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A free-form map of parameter names to their configured values.
    #[serde(rename = "mitigation_parameters")]
    pub mitigation_parameters: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The mitigation type to configure for the control.
    #[serde(rename = "mitigation_type")]
    pub mitigation_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceControlUpdateAttributes {
    pub fn new() -> GovernanceControlUpdateAttributes {
        GovernanceControlUpdateAttributes {
            detection_parameters: None,
            mitigation_parameters: None,
            mitigation_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn detection_parameters(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.detection_parameters = Some(value);
        self
    }

    pub fn mitigation_parameters(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.mitigation_parameters = Some(value);
        self
    }

    pub fn mitigation_type(mut self, value: String) -> Self {
        self.mitigation_type = Some(value);
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

impl Default for GovernanceControlUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GovernanceControlUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceControlUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceControlUpdateAttributesVisitor {
            type Value = GovernanceControlUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detection_parameters: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut mitigation_parameters: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut mitigation_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "detection_parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            detection_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigation_parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            mitigation_parameters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mitigation_type" => {
                            if v.is_null() {
                                continue;
                            }
                            mitigation_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GovernanceControlUpdateAttributes {
                    detection_parameters,
                    mitigation_parameters,
                    mitigation_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceControlUpdateAttributesVisitor)
    }
}
