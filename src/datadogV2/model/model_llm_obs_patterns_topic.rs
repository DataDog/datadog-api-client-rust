// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A topic discovered by an LLM Observability patterns run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsTopic {
    /// Timestamp when the topic was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Description of the topic.
    #[serde(rename = "description")]
    pub description: String,
    /// Timestamp when the topic was first seen.
    #[serde(rename = "first_seen_at")]
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    /// Level of the topic in the hierarchy. Level 0 is a leaf topic.
    #[serde(rename = "hierarchy_level")]
    pub hierarchy_level: i64,
    /// Unique identifier of the topic.
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the topic has been validated.
    #[serde(rename = "is_validated")]
    pub is_validated: bool,
    /// Name of the topic.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier of the parent topic. Empty for top-level topics.
    #[serde(rename = "parent_topic_id")]
    pub parent_topic_id: String,
    /// Number of data points assigned to the topic.
    #[serde(rename = "point_count")]
    pub point_count: i64,
    /// Identifier of the run that produced the topic.
    #[serde(rename = "run_id")]
    pub run_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsTopic {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        first_seen_at: chrono::DateTime<chrono::Utc>,
        hierarchy_level: i64,
        id: String,
        is_validated: bool,
        name: String,
        parent_topic_id: String,
        point_count: i64,
        run_id: String,
    ) -> LLMObsPatternsTopic {
        LLMObsPatternsTopic {
            created_at,
            description,
            first_seen_at,
            hierarchy_level,
            id,
            is_validated,
            name,
            parent_topic_id,
            point_count,
            run_id,
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

impl<'de> Deserialize<'de> for LLMObsPatternsTopic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsTopicVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsTopicVisitor {
            type Value = LLMObsPatternsTopic;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut first_seen_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut hierarchy_level: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut is_validated: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut parent_topic_id: Option<String> = None;
                let mut point_count: Option<i64> = None;
                let mut run_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen_at" => {
                            first_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hierarchy_level" => {
                            hierarchy_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_validated" => {
                            is_validated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parent_topic_id" => {
                            parent_topic_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "point_count" => {
                            point_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_id" => {
                            run_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let first_seen_at =
                    first_seen_at.ok_or_else(|| M::Error::missing_field("first_seen_at"))?;
                let hierarchy_level =
                    hierarchy_level.ok_or_else(|| M::Error::missing_field("hierarchy_level"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let is_validated =
                    is_validated.ok_or_else(|| M::Error::missing_field("is_validated"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let parent_topic_id =
                    parent_topic_id.ok_or_else(|| M::Error::missing_field("parent_topic_id"))?;
                let point_count =
                    point_count.ok_or_else(|| M::Error::missing_field("point_count"))?;
                let run_id = run_id.ok_or_else(|| M::Error::missing_field("run_id"))?;

                let content = LLMObsPatternsTopic {
                    created_at,
                    description,
                    first_seen_at,
                    hierarchy_level,
                    id,
                    is_validated,
                    name,
                    parent_topic_id,
                    point_count,
                    run_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsTopicVisitor)
    }
}
