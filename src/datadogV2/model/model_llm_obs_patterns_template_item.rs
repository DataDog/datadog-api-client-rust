// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single Agent Observability patterns template in a list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsTemplateItem {
    /// Instructions guiding how topics are clustered for configurations linked to this template.
    #[serde(rename = "clustering_instructions")]
    pub clustering_instructions: String,
    /// Timestamp when the template was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Human-readable description of what the template is for.
    #[serde(rename = "description")]
    pub description: String,
    /// Prefix merged into the query that selects spans for configurations linked to this template.
    #[serde(rename = "evp_query_prefix")]
    pub evp_query_prefix: String,
    /// Unique identifier of the template.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the template.
    #[serde(rename = "name")]
    pub name: String,
    /// Scope the template applies to.
    #[serde(rename = "scope")]
    pub scope: String,
    /// Template used to guide topic generation for configurations linked to this template.
    #[serde(rename = "template")]
    pub template: String,
    /// Default lookback window, in milliseconds, for configurations linked to this template.
    #[serde(rename = "time_frame_duration_ms")]
    pub time_frame_duration_ms: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsTemplateItem {
    pub fn new(
        clustering_instructions: String,
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        evp_query_prefix: String,
        id: String,
        name: String,
        scope: String,
        template: String,
        time_frame_duration_ms: i64,
    ) -> LLMObsPatternsTemplateItem {
        LLMObsPatternsTemplateItem {
            clustering_instructions,
            created_at,
            description,
            evp_query_prefix,
            id,
            name,
            scope,
            template,
            time_frame_duration_ms,
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

impl<'de> Deserialize<'de> for LLMObsPatternsTemplateItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsTemplateItemVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsTemplateItemVisitor {
            type Value = LLMObsPatternsTemplateItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut clustering_instructions: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut evp_query_prefix: Option<String> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut scope: Option<String> = None;
                let mut template: Option<String> = None;
                let mut time_frame_duration_ms: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "clustering_instructions" => {
                            clustering_instructions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evp_query_prefix" => {
                            evp_query_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template" => {
                            template = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time_frame_duration_ms" => {
                            time_frame_duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let clustering_instructions = clustering_instructions
                    .ok_or_else(|| M::Error::missing_field("clustering_instructions"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let evp_query_prefix =
                    evp_query_prefix.ok_or_else(|| M::Error::missing_field("evp_query_prefix"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let template = template.ok_or_else(|| M::Error::missing_field("template"))?;
                let time_frame_duration_ms = time_frame_duration_ms
                    .ok_or_else(|| M::Error::missing_field("time_frame_duration_ms"))?;

                let content = LLMObsPatternsTemplateItem {
                    clustering_instructions,
                    created_at,
                    description,
                    evp_query_prefix,
                    id,
                    name,
                    scope,
                    template,
                    time_frame_duration_ms,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsTemplateItemVisitor)
    }
}
