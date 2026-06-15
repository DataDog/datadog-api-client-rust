// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability patterns clustered points response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsClusteredPointsResponseAttributes {
    /// Pagination token for the next page of points. Null if there are no more pages.
    #[serialize_always]
    #[serde(rename = "next_page_token")]
    pub next_page_token: Option<String>,
    /// List of clustered points.
    #[serde(rename = "points")]
    pub points: Vec<crate::datadogV2::model::LLMObsPatternsClusteredPoint>,
    /// Identifier of the topic the points belong to.
    #[serde(rename = "topic_id")]
    pub topic_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsClusteredPointsResponseAttributes {
    pub fn new(
        next_page_token: Option<String>,
        points: Vec<crate::datadogV2::model::LLMObsPatternsClusteredPoint>,
        topic_id: String,
    ) -> LLMObsPatternsClusteredPointsResponseAttributes {
        LLMObsPatternsClusteredPointsResponseAttributes {
            next_page_token,
            points,
            topic_id,
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

impl<'de> Deserialize<'de> for LLMObsPatternsClusteredPointsResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsClusteredPointsResponseAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsClusteredPointsResponseAttributesVisitor {
            type Value = LLMObsPatternsClusteredPointsResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_page_token: Option<Option<String>> = None;
                let mut points: Option<Vec<crate::datadogV2::model::LLMObsPatternsClusteredPoint>> =
                    None;
                let mut topic_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_page_token" => {
                            next_page_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "points" => {
                            points = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "topic_id" => {
                            topic_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let next_page_token =
                    next_page_token.ok_or_else(|| M::Error::missing_field("next_page_token"))?;
                let points = points.ok_or_else(|| M::Error::missing_field("points"))?;
                let topic_id = topic_id.ok_or_else(|| M::Error::missing_field("topic_id"))?;

                let content = LLMObsPatternsClusteredPointsResponseAttributes {
                    next_page_token,
                    points,
                    topic_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsClusteredPointsResponseAttributesVisitor)
    }
}
