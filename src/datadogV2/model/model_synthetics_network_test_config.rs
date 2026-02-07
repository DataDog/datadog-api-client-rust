// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration object for a Network Path test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsNetworkTestConfig {
    /// Array of assertions used for the test.
    #[serde(rename = "assertions")]
    pub assertions: Option<Vec<crate::datadogV2::model::SyntheticsNetworkAssertion>>,
    /// Object describing the request for a Network Path test.
    #[serde(rename = "request")]
    pub request: Option<crate::datadogV2::model::SyntheticsNetworkTestRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsNetworkTestConfig {
    pub fn new() -> SyntheticsNetworkTestConfig {
        SyntheticsNetworkTestConfig {
            assertions: None,
            request: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assertions(
        mut self,
        value: Vec<crate::datadogV2::model::SyntheticsNetworkAssertion>,
    ) -> Self {
        self.assertions = Some(value);
        self
    }

    pub fn request(mut self, value: crate::datadogV2::model::SyntheticsNetworkTestRequest) -> Self {
        self.request = Some(value);
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

impl Default for SyntheticsNetworkTestConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsNetworkTestConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsNetworkTestConfigVisitor;
        impl<'a> Visitor<'a> for SyntheticsNetworkTestConfigVisitor {
            type Value = SyntheticsNetworkTestConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assertions: Option<
                    Vec<crate::datadogV2::model::SyntheticsNetworkAssertion>,
                > = None;
                let mut request: Option<crate::datadogV2::model::SyntheticsNetworkTestRequest> =
                    None;
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
                        "request" => {
                            if v.is_null() {
                                continue;
                            }
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsNetworkTestConfig {
                    assertions,
                    request,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsNetworkTestConfigVisitor)
    }
}
