// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Handshake request and response for protocol-level tests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultHandshake {
    /// Details of the outgoing request made during the test execution.
    #[serde(rename = "request")]
    pub request: Option<crate::datadogV2::model::SyntheticsTestResultRequestInfo>,
    /// Details of the response received during the test execution.
    #[serde(rename = "response")]
    pub response: Option<crate::datadogV2::model::SyntheticsTestResultResponseInfo>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultHandshake {
    pub fn new() -> SyntheticsTestResultHandshake {
        SyntheticsTestResultHandshake {
            request: None,
            response: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn request(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultRequestInfo,
    ) -> Self {
        self.request = Some(value);
        self
    }

    pub fn response(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultResponseInfo,
    ) -> Self {
        self.response = Some(value);
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

impl Default for SyntheticsTestResultHandshake {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultHandshake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultHandshakeVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultHandshakeVisitor {
            type Value = SyntheticsTestResultHandshake;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut request: Option<crate::datadogV2::model::SyntheticsTestResultRequestInfo> =
                    None;
                let mut response: Option<
                    crate::datadogV2::model::SyntheticsTestResultResponseInfo,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "request" => {
                            if v.is_null() {
                                continue;
                            }
                            request = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "response" => {
                            if v.is_null() {
                                continue;
                            }
                            response = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultHandshake {
                    request,
                    response,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultHandshakeVisitor)
    }
}
