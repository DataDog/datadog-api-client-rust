// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Variables captured during a test step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultVariables {
    /// Variables defined in the test configuration.
    #[serde(rename = "config")]
    pub config: Option<Vec<crate::datadogV2::model::SyntheticsTestResultVariable>>,
    /// Variables extracted during the test execution.
    #[serde(rename = "extracted")]
    pub extracted: Option<Vec<crate::datadogV2::model::SyntheticsTestResultVariable>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultVariables {
    pub fn new() -> SyntheticsTestResultVariables {
        SyntheticsTestResultVariables {
            config: None,
            extracted: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn config(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultVariable>,
    ) -> Self {
        self.config = Some(value);
        self
    }

    pub fn extracted(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsTestResultVariable>,
    ) -> Self {
        self.extracted = Some(value);
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

impl Default for SyntheticsTestResultVariables {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultVariables {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultVariablesVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultVariablesVisitor {
            type Value = SyntheticsTestResultVariables;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<Vec<crate::datadogV2::model::SyntheticsTestResultVariable>> =
                    None;
                let mut extracted: Option<
                    Vec<crate::datadogV2::model::SyntheticsTestResultVariable>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extracted" => {
                            if v.is_null() {
                                continue;
                            }
                            extracted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultVariables {
                    config,
                    extracted,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultVariablesVisitor)
    }
}
