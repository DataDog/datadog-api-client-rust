// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The incident impact's attributes for a create request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentImpactCreateAttributes {
    /// Description of the impact.
    #[serde(rename = "description")]
    pub description: String,
    /// Timestamp when the impact ended.
    #[serde(rename = "end_at", default, with = "::serde_with::rust::double_option")]
    pub end_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// An object mapping impact field names to field values.
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option")]
    pub fields: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Timestamp when the impact started.
    #[serde(rename = "start_at")]
    pub start_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentImpactCreateAttributes {
    pub fn new(
        description: String,
        start_at: chrono::DateTime<chrono::Utc>,
    ) -> IncidentImpactCreateAttributes {
        IncidentImpactCreateAttributes {
            description,
            end_at: None,
            fields: None,
            start_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for IncidentImpactCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentImpactCreateAttributesVisitor;
        impl<'a> Visitor<'a> for IncidentImpactCreateAttributesVisitor {
            type Value = IncidentImpactCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut end_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut fields: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut start_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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

                let content = IncidentImpactCreateAttributes {
                    description,
                    end_at,
                    fields,
                    start_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentImpactCreateAttributesVisitor)
    }
}
