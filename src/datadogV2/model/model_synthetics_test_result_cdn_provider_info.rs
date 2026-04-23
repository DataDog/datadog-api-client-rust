// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CDN provider details inferred from response headers.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultCdnProviderInfo {
    /// Cache status reported by the CDN for the response.
    #[serde(rename = "cache")]
    pub cache: Option<crate::datadogV2::model::SyntheticsTestResultCdnCacheStatus>,
    /// Name of the CDN provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultCdnProviderInfo {
    pub fn new() -> SyntheticsTestResultCdnProviderInfo {
        SyntheticsTestResultCdnProviderInfo {
            cache: None,
            provider: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cache(
        mut self,
        value: crate::datadogV2::model::SyntheticsTestResultCdnCacheStatus,
    ) -> Self {
        self.cache = Some(value);
        self
    }

    pub fn provider(mut self, value: String) -> Self {
        self.provider = Some(value);
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

impl Default for SyntheticsTestResultCdnProviderInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultCdnProviderInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultCdnProviderInfoVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultCdnProviderInfoVisitor {
            type Value = SyntheticsTestResultCdnProviderInfo;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cache: Option<crate::datadogV2::model::SyntheticsTestResultCdnCacheStatus> =
                    None;
                let mut provider: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cache" => {
                            if v.is_null() {
                                continue;
                            }
                            cache = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultCdnProviderInfo {
                    cache,
                    provider,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultCdnProviderInfoVisitor)
    }
}
