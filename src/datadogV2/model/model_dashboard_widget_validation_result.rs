// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Validation result for one dashboard widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardWidgetValidationResult {
    /// Validation error message, when the widget is invalid.
    #[serialize_always]
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Path to the invalid value, when available.
    #[serialize_always]
    #[serde(rename = "error_path")]
    pub error_path: Option<String>,
    /// Whether the widget passed validation.
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
    /// Type of the validated widget, when available.
    #[serialize_always]
    #[serde(rename = "widget_type")]
    pub widget_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardWidgetValidationResult {
    pub fn new(
        error_message: Option<String>,
        error_path: Option<String>,
        is_valid: bool,
        widget_type: Option<String>,
    ) -> DashboardWidgetValidationResult {
        DashboardWidgetValidationResult {
            error_message,
            error_path,
            is_valid,
            widget_type,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DashboardWidgetValidationResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardWidgetValidationResultVisitor;
        impl<'a> Visitor<'a> for DashboardWidgetValidationResultVisitor {
            type Value = DashboardWidgetValidationResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_message: Option<Option<String>> = None;
                let mut error_path: Option<Option<String>> = None;
                let mut is_valid: Option<bool> = None;
                let mut widget_type: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_message" => {
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_path" => {
                            error_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_valid" => {
                            is_valid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widget_type" => {
                            widget_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error_message =
                    error_message.ok_or_else(|| M::Error::missing_field("error_message"))?;
                let error_path = error_path.ok_or_else(|| M::Error::missing_field("error_path"))?;
                let is_valid = is_valid.ok_or_else(|| M::Error::missing_field("is_valid"))?;
                let widget_type =
                    widget_type.ok_or_else(|| M::Error::missing_field("widget_type"))?;

                let content = DashboardWidgetValidationResult {
                    error_message,
                    error_path,
                    is_valid,
                    widget_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardWidgetValidationResultVisitor)
    }
}
