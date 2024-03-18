// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration object for a Synthetic browser test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBrowserTestConfig {
    /// Array of assertions used for the test.
    #[serde(rename = "assertions")]
    pub assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
    /// Array of variables used for the test.
    #[serde(rename = "configVariables")]
    pub config_variables: Option<Vec<crate::datadogV1::model::SyntheticsConfigVariable>>,
    /// Object describing the Synthetic test request.
    #[serde(rename = "request")]
    pub request: crate::datadogV1::model::SyntheticsTestRequest,
    /// Cookies to be used for the request, using the [Set-Cookie](<https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie>) syntax.
    #[serde(rename = "setCookie")]
    pub set_cookie: Option<String>,
    /// Array of variables used for the test steps.
    #[serde(rename = "variables")]
    pub variables: Option<Vec<crate::datadogV1::model::SyntheticsBrowserVariable>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBrowserTestConfig {
    pub fn new(
        assertions: Vec<crate::datadogV1::model::SyntheticsAssertion>,
        request: crate::datadogV1::model::SyntheticsTestRequest,
    ) -> SyntheticsBrowserTestConfig {
        SyntheticsBrowserTestConfig {
            assertions,
            config_variables: None,
            request,
            set_cookie: None,
            variables: None,
            _unparsed: false,
        }
    }

    pub fn config_variables(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsConfigVariable>,
    ) -> Self {
        self.config_variables = Some(value);
        self
    }

    pub fn set_cookie(mut self, value: String) -> Self {
        self.set_cookie = Some(value);
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

impl<'de> Deserialize<'de> for SyntheticsBrowserTestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBrowserTestConfigVisitor;
        impl<'a> Visitor<'a> for SyntheticsBrowserTestConfigVisitor {
            type Value = SyntheticsBrowserTestConfig;

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
                let mut set_cookie: Option<String> = None;
                let mut variables: Option<Vec<crate::datadogV1::model::SyntheticsBrowserVariable>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assertions" => {
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
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "setCookie" => {
                            if v.is_null() {
                                continue;
                            }
                            set_cookie = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let assertions = assertions.ok_or_else(|| M::Error::missing_field("assertions"))?;
                let request = request.ok_or_else(|| M::Error::missing_field("request"))?;

                let content = SyntheticsBrowserTestConfig {
                    assertions,
                    config_variables,
                    request,
                    set_cookie,
                    variables,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBrowserTestConfigVisitor)
    }
}
