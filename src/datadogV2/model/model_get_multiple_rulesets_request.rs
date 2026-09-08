// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The request payload for retrieving rules for multiple rulesets in a single batch call.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetMultipleRulesetsRequest {
    /// CSRF token for security, sent by browser-based clients. Ignored by the API when absent.
    #[serde(rename = "_authentication_token")]
    pub _authentication_token: Option<String>,
    /// The primary data object in the get-multiple-rulesets request, containing request attributes and resource type.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GetMultipleRulesetsRequestData>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetMultipleRulesetsRequest {
    pub fn new() -> GetMultipleRulesetsRequest {
        GetMultipleRulesetsRequest {
            _authentication_token: None,
            data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn _authentication_token(mut self, value: String) -> Self {
        self._authentication_token = Some(value);
        self
    }

    pub fn data(mut self, value: crate::datadogV2::model::GetMultipleRulesetsRequestData) -> Self {
        self.data = Some(value);
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

impl Default for GetMultipleRulesetsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetMultipleRulesetsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetMultipleRulesetsRequestVisitor;
        impl<'a> Visitor<'a> for GetMultipleRulesetsRequestVisitor {
            type Value = GetMultipleRulesetsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut _authentication_token: Option<String> = None;
                let mut data: Option<crate::datadogV2::model::GetMultipleRulesetsRequestData> =
                    None;
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
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetMultipleRulesetsRequest {
                    _authentication_token,
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetMultipleRulesetsRequestVisitor)
    }
}
