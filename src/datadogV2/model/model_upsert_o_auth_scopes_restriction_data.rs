// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data object of an upsert OAuth2 scopes restriction request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertOAuthScopesRestrictionData {
    /// Attributes of an upsert OAuth2 scopes restriction request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::UpsertOAuthScopesRestrictionDataAttributes>,
    /// JSON:API resource type for an upsert OAuth2 client scopes restriction request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::UpsertOAuthScopesRestrictionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertOAuthScopesRestrictionData {
    pub fn new(
        type_: crate::datadogV2::model::UpsertOAuthScopesRestrictionType,
    ) -> UpsertOAuthScopesRestrictionData {
        UpsertOAuthScopesRestrictionData {
            attributes: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::UpsertOAuthScopesRestrictionDataAttributes,
    ) -> Self {
        self.attributes = Some(value);
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

impl<'de> Deserialize<'de> for UpsertOAuthScopesRestrictionData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertOAuthScopesRestrictionDataVisitor;
        impl<'a> Visitor<'a> for UpsertOAuthScopesRestrictionDataVisitor {
            type Value = UpsertOAuthScopesRestrictionData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::UpsertOAuthScopesRestrictionDataAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::UpsertOAuthScopesRestrictionType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::UpsertOAuthScopesRestrictionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = UpsertOAuthScopesRestrictionData {
                    attributes,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertOAuthScopesRestrictionDataVisitor)
    }
}
