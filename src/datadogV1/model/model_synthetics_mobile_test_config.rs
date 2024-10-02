// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration object for a Synthetic mobile test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileTestConfig {
    /// Initial application arguments for a mobile test.
    #[serde(rename = "initialApplicationArguments")]
    pub initial_application_arguments:
        Option<crate::datadogV1::model::SyntheticsMobileTestInitialApplicationArguments>,
    /// Array of variables used for the test steps.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileTestConfig {
    pub fn new() -> SyntheticsMobileTestConfig {
        SyntheticsMobileTestConfig {
            initial_application_arguments: None,
            variables: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn initial_application_arguments(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestInitialApplicationArguments,
    ) -> Self {
        self.initial_application_arguments = Some(value);
        self
    }

    pub fn variables(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsConfigVariable>,
    ) -> Self {
        self.variables = Some(value);
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

impl Default for SyntheticsMobileTestConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileTestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileTestConfigVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileTestConfigVisitor {
            type Value = SyntheticsMobileTestConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut initial_application_arguments: Option<
                    crate::datadogV1::model::SyntheticsMobileTestInitialApplicationArguments,
                > = None;
                let mut variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "initialApplicationArguments" => {
                            if v.is_null() {
                                continue;
                            }
                            initial_application_arguments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileTestConfig {
                    initial_application_arguments,
                    variables,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileTestConfigVisitor)
    }
}
