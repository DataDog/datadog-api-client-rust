// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an incident responder in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentResponderDataAttributesResponse {
    /// Timestamp when the responder was created.
    #[serde(rename = "created")]
    pub created: chrono::DateTime<chrono::Utc>,
    /// The external ID of the responder.
    #[serde(
        rename = "external_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub external_id: Option<Option<String>>,
    /// The external source of the responder.
    #[serde(
        rename = "external_source",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub external_source: Option<Option<String>>,
    /// Whether this responder counts toward billing.
    #[serde(rename = "is_billable")]
    pub is_billable: bool,
    /// Timestamp when the responder was last active.
    #[serde(
        rename = "last_active",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_active: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Additional metadata for the responder.
    #[serde(rename = "meta", default, with = "::serde_with::rust::double_option")]
    pub meta: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Timestamp when the responder was last modified.
    #[serde(rename = "modified")]
    pub modified: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentResponderDataAttributesResponse {
    pub fn new(
        created: chrono::DateTime<chrono::Utc>,
        is_billable: bool,
        modified: chrono::DateTime<chrono::Utc>,
    ) -> IncidentResponderDataAttributesResponse {
        IncidentResponderDataAttributesResponse {
            created,
            external_id: None,
            external_source: None,
            is_billable,
            last_active: None,
            meta: None,
            modified,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn external_id(mut self, value: Option<String>) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn external_source(mut self, value: Option<String>) -> Self {
        self.external_source = Some(value);
        self
    }

    pub fn last_active(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.last_active = Some(value);
        self
    }

    pub fn meta(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for IncidentResponderDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentResponderDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for IncidentResponderDataAttributesResponseVisitor {
            type Value = IncidentResponderDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut external_id: Option<Option<String>> = None;
                let mut external_source: Option<Option<String>> = None;
                let mut is_billable: Option<bool> = None;
                let mut last_active: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut meta: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created" => {
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_id" => {
                            external_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "external_source" => {
                            external_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_billable" => {
                            is_billable =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_active" => {
                            last_active =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created = created.ok_or_else(|| M::Error::missing_field("created"))?;
                let is_billable =
                    is_billable.ok_or_else(|| M::Error::missing_field("is_billable"))?;
                let modified = modified.ok_or_else(|| M::Error::missing_field("modified"))?;

                let content = IncidentResponderDataAttributesResponse {
                    created,
                    external_id,
                    external_source,
                    is_billable,
                    last_active,
                    meta,
                    modified,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentResponderDataAttributesResponseVisitor)
    }
}
