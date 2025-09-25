// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident impact's attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentImpactAttributes {
    /// Timestamp when the impact was created.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Description of the impact.
    #[serde(rename = "description")]
    pub description: String,
    /// Timestamp when the impact ended.
    #[serde(rename = "end_at", default, with = "::serde_with::rust::double_option")]
    pub end_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// An object mapping impact field names to field values.
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option")]
    pub fields: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// The type of impact.
    #[serde(rename = "impact_type")]
    pub impact_type: Option<String>,
    /// Timestamp when the impact was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp representing when the impact started.
    #[serde(rename = "start_at")]
    pub start_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentImpactAttributes {
    pub fn new(
        description: String,
        start_at: chrono::DateTime<chrono::Utc>,
    ) -> IncidentImpactAttributes {
        IncidentImpactAttributes {
            created: None,
            description,
            end_at: None,
            fields: None,
            impact_type: None,
            modified: None,
            start_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn end_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.end_at = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn impact_type(mut self, value: String) -> Self {
        self.impact_type = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
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

impl<'de> Deserialize<'de> for IncidentImpactAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentImpactAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentImpactAttributesVisitor {
            type Value = IncidentImpactAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut end_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut fields: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut impact_type: Option<String> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut start_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_at" => {
                            end_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fields" => {
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impact_type" => {
                            if v.is_null() {
                                continue;
                            }
                            impact_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_at" => {
                            start_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let start_at = start_at.ok_or_else(|| M::Error::missing_field("start_at"))?;

                let content = IncidentImpactAttributes {
                    created,
                    description,
                    end_at,
                    fields,
                    impact_type,
                    modified,
                    start_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentImpactAttributesVisitor)
    }
}
