// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an LLM Observability prompt registry entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPromptDataAttributes {
    /// UUID of the user who authored the prompt.
    #[serde(rename = "author")]
    pub author: Option<String>,
    /// Timestamp when the prompt was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Source that created the prompt, such as `ui-registry`, `sdk-registry`, or `sdk-instrumentation`.
    #[serde(rename = "created_from")]
    pub created_from: String,
    /// Datasets observed in runs associated with this prompt.
    #[serde(rename = "datasets")]
    pub datasets: Option<Vec<crate::datadogV2::model::LLMObsPromptDataset>>,
    /// Description of the prompt.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Source prompt from which this prompt was extracted, when applicable.
    #[serde(rename = "extracted_from")]
    pub extracted_from: Option<String>,
    /// Whether the prompt is a registry entry (as opposed to a code-discovered prompt).
    #[serde(rename = "in_registry")]
    pub in_registry: bool,
    /// Timestamp of the most recent observed run of this prompt.
    #[serde(rename = "last_seen_at")]
    pub last_seen_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp when the most recent version of the prompt was created.
    #[serde(rename = "last_version_created_at")]
    pub last_version_created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The ML application this prompt is associated with.
    #[serde(rename = "ml_app")]
    pub ml_app: Option<String>,
    /// ML applications observed running this prompt.
    #[serde(rename = "ml_apps")]
    pub ml_apps: Option<Vec<String>>,
    /// Number of versions of the prompt.
    #[serde(rename = "num_versions")]
    pub num_versions: i64,
    /// Customer-provided identifier of the prompt.
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    /// Whether the prompt was created from the registry or discovered from observed LLM calls.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::LLMObsPromptResponseSource,
    /// Tags observed on runs of this prompt.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Title of the prompt.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPromptDataAttributes {
    pub fn new(
        created_from: String,
        in_registry: bool,
        num_versions: i64,
        prompt_id: String,
        source: crate::datadogV2::model::LLMObsPromptResponseSource,
    ) -> LLMObsPromptDataAttributes {
        LLMObsPromptDataAttributes {
            author: None,
            created_at: None,
            created_from,
            datasets: None,
            description: None,
            extracted_from: None,
            in_registry,
            last_seen_at: None,
            last_version_created_at: None,
            ml_app: None,
            ml_apps: None,
            num_versions,
            prompt_id,
            source,
            tags: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: String) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn datasets(mut self, value: Vec<crate::datadogV2::model::LLMObsPromptDataset>) -> Self {
        self.datasets = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn extracted_from(mut self, value: String) -> Self {
        self.extracted_from = Some(value);
        self
    }

    pub fn last_seen_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn last_version_created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_version_created_at = Some(value);
        self
    }

    pub fn ml_app(mut self, value: String) -> Self {
        self.ml_app = Some(value);
        self
    }

    pub fn ml_apps(mut self, value: Vec<String>) -> Self {
        self.ml_apps = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsPromptDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPromptDataAttributesVisitor;
        impl<'a> Visitor<'a> for LLMObsPromptDataAttributesVisitor {
            type Value = LLMObsPromptDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_from: Option<String> = None;
                let mut datasets: Option<Vec<crate::datadogV2::model::LLMObsPromptDataset>> = None;
                let mut description: Option<String> = None;
                let mut extracted_from: Option<String> = None;
                let mut in_registry: Option<bool> = None;
                let mut last_seen_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut last_version_created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut ml_app: Option<String> = None;
                let mut ml_apps: Option<Vec<String>> = None;
                let mut num_versions: Option<i64> = None;
                let mut prompt_id: Option<String> = None;
                let mut source: Option<crate::datadogV2::model::LLMObsPromptResponseSource> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_from" => {
                            created_from =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "datasets" => {
                            if v.is_null() {
                                continue;
                            }
                            datasets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extracted_from" => {
                            if v.is_null() {
                                continue;
                            }
                            extracted_from =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "in_registry" => {
                            in_registry =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_seen_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_version_created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_version_created_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ml_app" => {
                            if v.is_null() {
                                continue;
                            }
                            ml_app = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ml_apps" => {
                            if v.is_null() {
                                continue;
                            }
                            ml_apps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "num_versions" => {
                            num_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prompt_id" => {
                            prompt_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::LLMObsPromptResponseSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_from =
                    created_from.ok_or_else(|| M::Error::missing_field("created_from"))?;
                let in_registry =
                    in_registry.ok_or_else(|| M::Error::missing_field("in_registry"))?;
                let num_versions =
                    num_versions.ok_or_else(|| M::Error::missing_field("num_versions"))?;
                let prompt_id = prompt_id.ok_or_else(|| M::Error::missing_field("prompt_id"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = LLMObsPromptDataAttributes {
                    author,
                    created_at,
                    created_from,
                    datasets,
                    description,
                    extracted_from,
                    in_registry,
                    last_seen_at,
                    last_version_created_at,
                    ml_app,
                    ml_apps,
                    num_versions,
                    prompt_id,
                    source,
                    tags,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPromptDataAttributesVisitor)
    }
}
