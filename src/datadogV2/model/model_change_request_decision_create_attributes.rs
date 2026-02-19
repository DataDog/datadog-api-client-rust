// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a change request decision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeRequestDecisionCreateAttributes {
    /// The status of a change request decision.
    #[serde(rename = "change_request_status")]
    pub change_request_status: Option<crate::datadogV2::model::ChangeRequestDecisionStatusType>,
    /// The reason for requesting the decision.
    #[serde(rename = "request_reason")]
    pub request_reason: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeRequestDecisionCreateAttributes {
    pub fn new() -> ChangeRequestDecisionCreateAttributes {
        ChangeRequestDecisionCreateAttributes {
            change_request_status: None,
            request_reason: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_request_status(
        mut self,
        value: crate::datadogV2::model::ChangeRequestDecisionStatusType,
    ) -> Self {
        self.change_request_status = Some(value);
        self
    }

    pub fn request_reason(mut self, value: String) -> Self {
        self.request_reason = Some(value);
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

impl Default for ChangeRequestDecisionCreateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChangeRequestDecisionCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeRequestDecisionCreateAttributesVisitor;
        impl<'a> Visitor<'a> for ChangeRequestDecisionCreateAttributesVisitor {
            type Value = ChangeRequestDecisionCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_request_status: Option<
                    crate::datadogV2::model::ChangeRequestDecisionStatusType,
                > = None;
                let mut request_reason: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "change_request_status" => {
                            if v.is_null() {
                                continue;
                            }
                            change_request_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _change_request_status) = change_request_status {
                                match _change_request_status {
                                    crate::datadogV2::model::ChangeRequestDecisionStatusType::UnparsedObject(_change_request_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "request_reason" => {
                            if v.is_null() {
                                continue;
                            }
                            request_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChangeRequestDecisionCreateAttributes {
                    change_request_status,
                    request_reason,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeRequestDecisionCreateAttributesVisitor)
    }
}
