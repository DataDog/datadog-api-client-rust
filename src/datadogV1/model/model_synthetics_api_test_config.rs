// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration object for a Synthetic API test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAPITestConfig {
    /// Array of assertions used for the test. Required for single API tests.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<crate::datadogV1::model::SyntheticsAssertion>>,
    /// Array of variables used for the test.
    #[serde(rename = "configVariables")]
    pub config_variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>>,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: Option<crate::datadogV1::model::SyntheticsTestRequest>,
    /// When the test subtype is `multi`, the steps of the test.
    #[serde(rename = "steps")]
    pub steps: Option<Vec<crate::datadogV1::model::SyntheticsAPIStep>>,
    /// Variables defined from JavaScript code.
    #[serde(rename = "variablesFromScript")]
    pub variables_from_script: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAPITestConfig {
    pub fn new() -> SyntheticsAPITestConfig {
        SyntheticsAPITestConfig {
            assertions: None,
            config_variables: None,
            request: None,
            steps: None,
            variables_from_script: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assertions(mut self, value: Vec<crate::datadogV1::model::SyntheticsAssertion>) -> Self {
        self.assertions = Some(value);
        self
    }

    pub fn config_variables(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsConfigVariable>,
    ) -> Self {
        self.config_variables = Some(value);
        self
    }

    pub fn request(mut self, value: crate::datadogV1::model::SyntheticsTestRequest) -> Self {
        self.request = Some(value);
        self
    }

    pub fn steps(mut self, value: Vec<crate::datadogV1::model::SyntheticsAPIStep>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn variables_from_script(mut self, value: String) -> Self {
        self.variables_from_script = Some(value);
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

impl Default for SyntheticsAPITestConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsAPITestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAPITestConfigVisitor;
        impl<'a> Visitor<'a> for SyntheticsAPITestConfigVisitor {
            type Value = SyntheticsAPITestConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assertions: Option<Vec<crate::datadogV1::model::SyntheticsAssertion>> =
                    None;
                let mut config_variables: Option<
                    Vec<crate::datadogV1::model::SyntheticsConfigVariable>,
                > = None;
                let mut request: Option<crate::datadogV1::model::SyntheticsTestRequest> = None;
                let mut steps: Option<Vec<crate::datadogV1::model::SyntheticsAPIStep>> = None;
                let mut variables_from_script: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assertions" => {
                            if v.is_null() {
                                continue;
                            }
                            assertions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "configVariables" => {
                            if v.is_null() {
                                continue;
                            }
                            config_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request" => {
                            if v.is_null() {
                                continue;
                            }
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            if v.is_null() {
                                continue;
                            }
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variablesFromScript" => {
                            if v.is_null() {
                                continue;
                            }
                            variables_from_script =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsAPITestConfig {
                    assertions,
                    config_variables,
                    request,
                    steps,
                    variables_from_script,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAPITestConfigVisitor)
    }
}
