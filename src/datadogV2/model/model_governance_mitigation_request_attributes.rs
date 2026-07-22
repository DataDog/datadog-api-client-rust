// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a governance mitigation request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceMitigationRequestAttributes {
    /// The identifiers of the detections to mitigate in this request.
    #[serde(rename = "detection_ids")]
    pub detection_ids: Option<Vec<String>>,
    /// The detection type whose detections should be mitigated.
    #[serde(rename = "detection_type")]
    pub detection_type: Option<String>,
    /// A free-form map of parameter names to their configured values.
    #[serde(rename = "mitigation_parameters")]
    pub mitigation_parameters: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The mitigation to apply to the selected detections. Defaults to the control's configured mitigation when omitted.
    #[serde(rename = "mitigation_type")]
    pub mitigation_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceMitigationRequestAttributes {
    pub fn new() -> GovernanceMitigationRequestAttributes {
        GovernanceMitigationRequestAttributes {
            detection_ids: None,
            detection_type: None,
            mitigation_parameters: None,
            mitigation_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn detection_ids(mut self, value: Vec<String>) -> Self {
        self.detection_ids = Some(value);
        self
    }

    pub fn detection_type(mut self, value: String) -> Self {
        self.detection_type = Some(value);
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

impl Default for GovernanceMitigationRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GovernanceMitigationRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceMitigationRequestAttributesVisitor;
        impl<'a> Visitor<'a> for GovernanceMitigationRequestAttributesVisitor {
            type Value = GovernanceMitigationRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut detection_ids: Option<Vec<String>> = None;
                let mut detection_type: Option<String> = None;
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
                        "detection_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            detection_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detection_type" => {
                            if v.is_null() {
                                continue;
                            }
                            detection_type =
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

                let content = GovernanceMitigationRequestAttributes {
                    detection_ids,
                    detection_type,
                    mitigation_parameters,
                    mitigation_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceMitigationRequestAttributesVisitor)
    }
}
