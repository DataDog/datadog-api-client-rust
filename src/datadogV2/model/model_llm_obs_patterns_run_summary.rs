// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary of an LLM Observability patterns run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsPatternsRunSummary {
    /// Timestamp when the run completed. Null if the run has not completed.
    #[serde(
        rename = "completed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Snapshot of the configuration used for a patterns run.
    #[serde(rename = "config_snapshot")]
    pub config_snapshot: Option<crate::datadogV2::model::LLMObsPatternsConfigSnapshot>,
    /// Timestamp when the run was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Unique identifier of the run.
    #[serde(rename = "id")]
    pub id: String,
    /// Status of the run.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsPatternsRunSummary {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        id: String,
        status: String,
    ) -> LLMObsPatternsRunSummary {
        LLMObsPatternsRunSummary {
            completed_at: None,
            config_snapshot: None,
            created_at,
            id,
            status,
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

impl<'de> Deserialize<'de> for LLMObsPatternsRunSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsPatternsRunSummaryVisitor;
        impl<'a> Visitor<'a> for LLMObsPatternsRunSummaryVisitor {
            type Value = LLMObsPatternsRunSummary;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut config_snapshot: Option<
                    crate::datadogV2::model::LLMObsPatternsConfigSnapshot,
                > = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<String> = None;
                let mut status: Option<String> = None;
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
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = LLMObsPatternsRunSummary {
                    completed_at,
                    config_snapshot,
                    created_at,
                    id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsPatternsRunSummaryVisitor)
    }
}
