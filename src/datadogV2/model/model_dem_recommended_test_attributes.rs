// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an AI-recommended synthetic test for a DEM journey.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemRecommendedTestAttributes {
    /// The browser test configuration that can be used to create the recommended test.
    #[serde(rename = "config")]
    pub config: std::collections::BTreeMap<String, serde_json::Value>,
    /// The time when the recommendation was generated.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The display name of the recommended test.
    #[serde(rename = "name")]
    pub name: String,
    /// The identifier of the validating sample run, when available.
    #[serde(rename = "result_id")]
    pub result_id: Option<String>,
    /// The RUM session identifier for the validating sample run, when available.
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    /// The pipeline that produced the recommendation.
    #[serde(rename = "source")]
    pub source: String,
    /// The type of synthetic test.
    #[serde(rename = "type")]
    pub type_: String,
    /// The variant associated with the recommendation, when applicable.
    #[serde(rename = "variant_id")]
    pub variant_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemRecommendedTestAttributes {
    pub fn new(
        config: std::collections::BTreeMap<String, serde_json::Value>,
        created_at: chrono::DateTime<chrono::Utc>,
        name: String,
        source: String,
        type_: String,
    ) -> DemRecommendedTestAttributes {
        DemRecommendedTestAttributes {
            config,
            created_at,
            name,
            result_id: None,
            session_id: None,
            source,
            type_,
            variant_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn result_id(mut self, value: String) -> Self {
        self.result_id = Some(value);
        self
    }

    pub fn session_id(mut self, value: String) -> Self {
        self.session_id = Some(value);
        self
    }

    pub fn variant_id(mut self, value: String) -> Self {
        self.variant_id = Some(value);
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

impl<'de> Deserialize<'de> for DemRecommendedTestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemRecommendedTestAttributesVisitor;
        impl<'a> Visitor<'a> for DemRecommendedTestAttributesVisitor {
            type Value = DemRecommendedTestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut result_id: Option<String> = None;
                let mut session_id: Option<String> = None;
                let mut source: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut variant_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result_id" => {
                            if v.is_null() {
                                continue;
                            }
                            result_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            if v.is_null() {
                                continue;
                            }
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let config = config.ok_or_else(|| M::Error::missing_field("config"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = DemRecommendedTestAttributes {
                    config,
                    created_at,
                    name,
                    result_id,
                    session_id,
                    source,
                    type_,
                    variant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemRecommendedTestAttributesVisitor)
    }
}
