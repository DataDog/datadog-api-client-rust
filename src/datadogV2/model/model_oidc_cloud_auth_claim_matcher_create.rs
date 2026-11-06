// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Claim condition for creating a provider or persona mapping. A token claim must match at least one configured value.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OIDCCloudAuthClaimMatcherCreate {
    /// Name of the token claim.
    #[serde(rename = "claim")]
    pub claim: String,
    /// Case-sensitive matching mode. Exact compares strings. Glob matches the entire value: * matches any number of characters, including /, and ? matches one character. Other characters are literal; escaping is not supported.
    #[serde(rename = "mode")]
    pub mode: Option<crate::datadogV2::model::OIDCCloudAuthClaimMatchMode>,
    /// Accepted values for this claim. Each value is limited to 500 UTF-8 bytes when creating the resource.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OIDCCloudAuthClaimMatcherCreate {
    pub fn new(claim: String, values: Vec<String>) -> OIDCCloudAuthClaimMatcherCreate {
        OIDCCloudAuthClaimMatcherCreate {
            claim,
            mode: None,
            values,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn mode(mut self, value: crate::datadogV2::model::OIDCCloudAuthClaimMatchMode) -> Self {
        self.mode = Some(value);
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

impl<'de> Deserialize<'de> for OIDCCloudAuthClaimMatcherCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OIDCCloudAuthClaimMatcherCreateVisitor;
        impl<'a> Visitor<'a> for OIDCCloudAuthClaimMatcherCreateVisitor {
            type Value = OIDCCloudAuthClaimMatcherCreate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut claim: Option<String> = None;
                let mut mode: Option<crate::datadogV2::model::OIDCCloudAuthClaimMatchMode> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "claim" => {
                            claim = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            if v.is_null() {
                                continue;
                            }
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::OIDCCloudAuthClaimMatchMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let claim = claim.ok_or_else(|| M::Error::missing_field("claim"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = OIDCCloudAuthClaimMatcherCreate {
                    claim,
                    mode,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OIDCCloudAuthClaimMatcherCreateVisitor)
    }
}
