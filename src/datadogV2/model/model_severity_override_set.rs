// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Applies a manual severity override to the findings.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SeverityOverrideSet {
    /// The action that applies a manual severity override.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::SeverityOverrideSetActionType,
    /// Additional information about the severity change. This field has a limit of 280 characters.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Severity to apply to the findings.
    /// `info` sets the lowest severity the finding type allows.
    #[serde(rename = "value")]
    pub value: crate::datadogV2::model::SeverityOverrideValue,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SeverityOverrideSet {
    pub fn new(
        action: crate::datadogV2::model::SeverityOverrideSetActionType,
        value: crate::datadogV2::model::SeverityOverrideValue,
    ) -> SeverityOverrideSet {
        SeverityOverrideSet {
            action,
            description: None,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for SeverityOverrideSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeverityOverrideSetVisitor;
        impl<'a> Visitor<'a> for SeverityOverrideSetVisitor {
            type Value = SeverityOverrideSet;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::SeverityOverrideSetActionType> =
                    None;
                let mut description: Option<String> = None;
                let mut value: Option<crate::datadogV2::model::SeverityOverrideValue> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::SeverityOverrideSetActionType::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV2::model::SeverityOverrideValue::UnparsedObject(_value) => {
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
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = SeverityOverrideSet {
                    action,
                    description,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SeverityOverrideSetVisitor)
    }
}
