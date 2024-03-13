// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration object for a Synthetic test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestConfig {
    /// Array of assertions used for the test. Required for single API tests.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<crate::datadogV1::model::SyntheticsAssertion>>,
    /// Array of variables used for the test.
    #[serde(rename = "configVariables")]
    pub config_variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>>,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: Option<crate::datadogV1::model::SyntheticsTestRequest>,
    /// Browser tests only - array of variables used for the test steps.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsBrowserVariable>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestConfig {
    pub fn new() -> SyntheticsTestConfig {
        SyntheticsTestConfig {
            assertions: None,
            config_variables: None,
            request: None,
            variables: None,
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

    pub fn variables(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsBrowserVariable>,
    ) -> Self {
        self.variables = Some(value);
        self
    }
}

impl Default for SyntheticsTestConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestConfigVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestConfigVisitor {
            type Value = SyntheticsTestConfig;

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
                let mut variables: Option<Vec<crate::datadogV1::model::SyntheticsBrowserVariable>> =
                    None;
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
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsTestConfig {
                    assertions,
                    config_variables,
                    request,
                    variables,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestConfigVisitor)
    }
}
