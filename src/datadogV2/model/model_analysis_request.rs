// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The request payload for running static analysis on source code.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisRequest {
    /// CSRF token for security, sent by browser-based clients. Ignored by the API when absent.
    #[serde(rename = "_authentication_token")]
    pub _authentication_token: Option<String>,
    /// The primary data object in the analysis request.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::AnalysisRequestData,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AnalysisRequest {
    pub fn new(data: crate::datadogV2::model::AnalysisRequestData) -> AnalysisRequest {
        AnalysisRequest {
            _authentication_token: None,
            data,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn _authentication_token(mut self, value: String) -> Self {
        self._authentication_token = Some(value);
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

impl<'de> Deserialize<'de> for AnalysisRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AnalysisRequestVisitor;
        impl<'a> Visitor<'a> for AnalysisRequestVisitor {
            type Value = AnalysisRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut _authentication_token: Option<String> = None;
                let mut data: Option<crate::datadogV2::model::AnalysisRequestData> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "_authentication_token" => {
                            if v.is_null() {
                                continue;
                            }
                            _authentication_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = AnalysisRequest {
                    _authentication_token,
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AnalysisRequestVisitor)
    }
}
