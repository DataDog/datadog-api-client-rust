// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request containing dashboard widgets and their layout context.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardWidgetValidationRequest {
    /// Layout type used to apply dashboard-specific widget layout validation.
    #[serde(rename = "layout_type")]
    pub layout_type: crate::datadogV2::model::DashboardWidgetValidationLayoutType,
    /// Reflow behavior used for an ordered dashboard.
    #[serde(rename = "reflow_type")]
    pub reflow_type: Option<crate::datadogV2::model::DashboardWidgetValidationReflowType>,
    /// Dashboard widgets to validate.
    #[serde(rename = "widgets")]
    pub widgets: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardWidgetValidationRequest {
    pub fn new(
        layout_type: crate::datadogV2::model::DashboardWidgetValidationLayoutType,
        widgets: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> DashboardWidgetValidationRequest {
        DashboardWidgetValidationRequest {
            layout_type,
            reflow_type: None,
            widgets,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reflow_type(
        mut self,
        value: crate::datadogV2::model::DashboardWidgetValidationReflowType,
    ) -> Self {
        self.reflow_type = Some(value);
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

impl<'de> Deserialize<'de> for DashboardWidgetValidationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardWidgetValidationRequestVisitor;
        impl<'a> Visitor<'a> for DashboardWidgetValidationRequestVisitor {
            type Value = DashboardWidgetValidationRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut layout_type: Option<
                    crate::datadogV2::model::DashboardWidgetValidationLayoutType,
                > = None;
                let mut reflow_type: Option<
                    crate::datadogV2::model::DashboardWidgetValidationReflowType,
                > = None;
                let mut widgets: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "layout_type" => {
                            layout_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _layout_type) = layout_type {
                                match _layout_type {
                                    crate::datadogV2::model::DashboardWidgetValidationLayoutType::UnparsedObject(_layout_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "reflow_type" => {
                            if v.is_null() {
                                continue;
                            }
                            reflow_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reflow_type) = reflow_type {
                                match _reflow_type {
                                    crate::datadogV2::model::DashboardWidgetValidationReflowType::UnparsedObject(_reflow_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "widgets" => {
                            widgets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let layout_type =
                    layout_type.ok_or_else(|| M::Error::missing_field("layout_type"))?;
                let widgets = widgets.ok_or_else(|| M::Error::missing_field("widgets"))?;

                let content = DashboardWidgetValidationRequest {
                    layout_type,
                    reflow_type,
                    widgets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardWidgetValidationRequestVisitor)
    }
}
