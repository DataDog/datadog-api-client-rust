// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A custom static analysis rule within a ruleset, as supplied in a create or update
/// request. Nested rules are sent flat, without a `data`/`type`/`attributes` envelope.
/// `id` and `name` are client-supplied and must match each other. The remaining members
/// are server-assigned and read-only; they are declared so that a ruleset previously
/// read back can be supplied unchanged.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomRule {
    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Creator identifier
    #[serde(rename = "created_by")]
    pub created_by: Option<String>,
    /// Rule identifier, which is the same as the rule name.
    #[serde(rename = "id")]
    pub id: String,
    /// A revision of a custom static analysis rule as embedded in a rule supplied by a create
    /// or update request. Nested revisions are sent flat, without a `data`/`type`/`attributes`
    /// envelope. `id`, `version_id`, `checksum`, `created_at` and `created_by` are server-assigned
    /// and read-only; they are declared so that a ruleset previously read back can be supplied
    /// unchanged.
    #[serde(rename = "last_revision")]
    pub last_revision: Option<crate::datadogV2::model::CustomRuleRevisionInput>,
    /// Rule name
    #[serde(rename = "name")]
    pub name: String,
    /// Revision history of the rule.
    #[serde(
        rename = "revisions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub revisions: Option<Option<Vec<crate::datadogV2::model::CustomRuleRevisionInput>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomRule {
    pub fn new(id: String, name: String) -> CustomRule {
        CustomRule {
            created_at: None,
            created_by: None,
            id,
            last_revision: None,
            name,
            revisions: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: String) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn last_revision(
        mut self,
        value: crate::datadogV2::model::CustomRuleRevisionInput,
    ) -> Self {
        self.last_revision = Some(value);
        self
    }

    pub fn revisions(
        mut self,
        value: Option<Vec<crate::datadogV2::model::CustomRuleRevisionInput>>,
    ) -> Self {
        self.revisions = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CustomRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomRuleVisitor;
        impl<'a> Visitor<'a> for CustomRuleVisitor {
            type Value = CustomRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut id: Option<String> = None;
                let mut last_revision: Option<crate::datadogV2::model::CustomRuleRevisionInput> =
                    None;
                let mut name: Option<String> = None;
                let mut revisions: Option<
                    Option<Vec<crate::datadogV2::model::CustomRuleRevisionInput>>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_revision" => {
                            if v.is_null() {
                                continue;
                            }
                            last_revision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revisions" => {
                            revisions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = CustomRule {
                    created_at,
                    created_by,
                    id,
                    last_revision,
                    name,
                    revisions,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomRuleVisitor)
    }
}
