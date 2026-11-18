// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a step suggestions request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFunnelStepSuggestionsRequestAttributes {
    /// Only use journeys from this RUM application.
    #[serde(rename = "application_id")]
    pub application_id: Option<String>,
    /// Order of the funnel steps, as step IDs joined by `->`.
    #[serde(rename = "expression")]
    pub expression: String,
    /// Map of step ID to the event query of the step. Steps are matched in the string sort
    /// order of their IDs, so use zero-padded IDs such as `node_01` and `node_02`. Without padding,
    /// `node_10` sorts before `node_2`.
    #[serde(rename = "steps")]
    pub steps: std::collections::BTreeMap<String, String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFunnelStepSuggestionsRequestAttributes {
    pub fn new(
        expression: String,
        steps: std::collections::BTreeMap<String, String>,
    ) -> ProductAnalyticsFunnelStepSuggestionsRequestAttributes {
        ProductAnalyticsFunnelStepSuggestionsRequestAttributes {
            application_id: None,
            expression,
            steps,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: String) -> Self {
        self.application_id = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsFunnelStepSuggestionsRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFunnelStepSuggestionsRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFunnelStepSuggestionsRequestAttributesVisitor {
            type Value = ProductAnalyticsFunnelStepSuggestionsRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut expression: Option<String> = None;
                let mut steps: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;
                let steps = steps.ok_or_else(|| M::Error::missing_field("steps"))?;

                let content = ProductAnalyticsFunnelStepSuggestionsRequestAttributes {
                    application_id,
                    expression,
                    steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFunnelStepSuggestionsRequestAttributesVisitor)
    }
}
