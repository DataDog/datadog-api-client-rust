// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a status page email access policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPageEmailAccessPolicyDataAttributes {
    /// Timestamp of when the email access policy was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The effect applied when an email address matches the policy. Deny policies take precedence over allow policies.
    #[serde(rename = "effect")]
    pub effect: crate::datadogV2::model::AccessPolicyEffect,
    /// The email address pattern matched by the policy.
    #[serde(rename = "pattern")]
    pub pattern: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPageEmailAccessPolicyDataAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        effect: crate::datadogV2::model::AccessPolicyEffect,
        pattern: String,
    ) -> StatusPageEmailAccessPolicyDataAttributes {
        StatusPageEmailAccessPolicyDataAttributes {
            created_at,
            effect,
            pattern,
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

impl<'de> Deserialize<'de> for StatusPageEmailAccessPolicyDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageEmailAccessPolicyDataAttributesVisitor;
        impl<'a> Visitor<'a> for StatusPageEmailAccessPolicyDataAttributesVisitor {
            type Value = StatusPageEmailAccessPolicyDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut effect: Option<crate::datadogV2::model::AccessPolicyEffect> = None;
                let mut pattern: Option<String> = None;
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
                        "effect" => {
                            effect = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _effect) = effect {
                                match _effect {
                                    crate::datadogV2::model::AccessPolicyEffect::UnparsedObject(
                                        _effect,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "pattern" => {
                            pattern = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let effect = effect.ok_or_else(|| M::Error::missing_field("effect"))?;
                let pattern = pattern.ok_or_else(|| M::Error::missing_field("pattern"))?;

                let content = StatusPageEmailAccessPolicyDataAttributes {
                    created_at,
                    effect,
                    pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageEmailAccessPolicyDataAttributesVisitor)
    }
}
