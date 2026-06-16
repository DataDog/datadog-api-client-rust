// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the topics, and the clustered points of their leaf topics,
/// discovered by an LLM Observability patterns run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsTopicsWithClusteredPointsResponse {
    /// Data object of an LLM Observability patterns topics-with-clustered-points response.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::LLMObsPatternsTopicsWithClusteredPointsResponseData,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsTopicsWithClusteredPointsResponse {
    pub fn new(
        data: crate::datadogV2::model::LLMObsPatternsTopicsWithClusteredPointsResponseData,
    ) -> LLMObsPatternsTopicsWithClusteredPointsResponse {
        LLMObsPatternsTopicsWithClusteredPointsResponse {
            data,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for LLMObsPatternsTopicsWithClusteredPointsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsTopicsWithClusteredPointsResponseVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsTopicsWithClusteredPointsResponseVisitor {
            type Value = LLMObsPatternsTopicsWithClusteredPointsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    crate::datadogV2::model::LLMObsPatternsTopicsWithClusteredPointsResponseData,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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

                let content = LLMObsPatternsTopicsWithClusteredPointsResponse {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsTopicsWithClusteredPointsResponseVisitor)
    }
}
