// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability patterns topics-with-clustered-points response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsTopicsWithClusteredPointsResponseAttributes {
    /// Timestamp when the run completed. Null if the run has not completed.
    #[serde(
        rename = "completed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Identifier of the configuration that produced the run.
    #[serde(rename = "config_id")]
    pub config_id: String,
    /// Snapshot of the configuration used for a patterns run.
    #[serde(rename = "config_snapshot")]
    pub config_snapshot: Option<crate::datadogV2::model::LLMObsPatternsConfigSnapshot>,
    /// Timestamp when the run was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Identifier of the run that completed immediately before this one. Empty if none.
    #[serde(rename = "previous_run_id")]
    pub previous_run_id: String,
    /// Identifier of the run that produced the topics.
    #[serde(rename = "run_id")]
    pub run_id: String,
    /// List of discovered topics with their clustered points.
    #[serde(rename = "topics")]
    pub topics: Vec<crate::datadogV2::model::LLMObsPatternsTopicWithClusteredPoints>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsTopicsWithClusteredPointsResponseAttributes {
    pub fn new(
        config_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        previous_run_id: String,
        run_id: String,
        topics: Vec<crate::datadogV2::model::LLMObsPatternsTopicWithClusteredPoints>,
    ) -> LLMObsPatternsTopicsWithClusteredPointsResponseAttributes {
        LLMObsPatternsTopicsWithClusteredPointsResponseAttributes {
            completed_at: None,
            config_id,
            config_snapshot: None,
            created_at,
            previous_run_id,
            run_id,
            topics,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn completed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn config_snapshot(
        mut self,
        value: crate::datadogV2::model::LLMObsPatternsConfigSnapshot,
    ) -> Self {
        self.config_snapshot = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsPatternsTopicsWithClusteredPointsResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsTopicsWithClusteredPointsResponseAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsTopicsWithClusteredPointsResponseAttributesVisitor {
            type Value = LLMObsPatternsTopicsWithClusteredPointsResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut config_id: Option<String> = None;
                let mut config_snapshot: Option<
                    crate::datadogV2::model::LLMObsPatternsConfigSnapshot,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut previous_run_id: Option<String> = None;
                let mut run_id: Option<String> = None;
                let mut topics: Option<
                    Vec<crate::datadogV2::model::LLMObsPatternsTopicWithClusteredPoints>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "completed_at" => {
                            completed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_id" => {
                            config_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "config_snapshot" => {
                            if v.is_null() {
                                continue;
                            }
                            config_snapshot =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "previous_run_id" => {
                            previous_run_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "run_id" => {
                            run_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "topics" => {
                            topics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config_id = config_id.ok_or_else(|| M::Error::missing_field("config_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let previous_run_id =
                    previous_run_id.ok_or_else(|| M::Error::missing_field("previous_run_id"))?;
                let run_id = run_id.ok_or_else(|| M::Error::missing_field("run_id"))?;
                let topics = topics.ok_or_else(|| M::Error::missing_field("topics"))?;

                let content = LLMObsPatternsTopicsWithClusteredPointsResponseAttributes {
                    completed_at,
                    config_id,
                    config_snapshot,
                    created_at,
                    previous_run_id,
                    run_id,
                    topics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(LLMObsPatternsTopicsWithClusteredPointsResponseAttributesVisitor)
    }
}
