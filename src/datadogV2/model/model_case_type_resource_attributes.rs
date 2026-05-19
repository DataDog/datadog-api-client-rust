// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a case type, which define a classification category for cases. Organizations use case types to model different workflows (for example, Security Incident, Bug Report, Change Request).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseTypeResourceAttributes {
    /// Timestamp when the case type was marked as deleted. A null value indicates the case type is active.
    #[serde(
        rename = "deleted_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A detailed description explaining when this case type should be used.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// An emoji icon representing the case type in the UI.
    #[serde(rename = "emoji")]
    pub emoji: Option<String>,
    /// The display name of the case type, shown in the Case Management UI when creating or viewing cases.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseTypeResourceAttributes {
    pub fn new(name: String) -> CaseTypeResourceAttributes {
        CaseTypeResourceAttributes {
            deleted_at: None,
            description: None,
            emoji: None,
            name,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deleted_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn emoji(mut self, value: String) -> Self {
        self.emoji = Some(value);
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

impl<'de> Deserialize<'de> for CaseTypeResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseTypeResourceAttributesVisitor;
        impl<'a> Visitor<'a> for CaseTypeResourceAttributesVisitor {
            type Value = CaseTypeResourceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut description: Option<String> = None;
                let mut emoji: Option<String> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "emoji" => {
                            if v.is_null() {
                                continue;
                            }
                            emoji = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CaseTypeResourceAttributes {
                    deleted_at,
                    description,
                    emoji,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseTypeResourceAttributesVisitor)
    }
}
