// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Facet data for user attributes of an incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentSearchResponseUserFacetData {
    /// Count of the facet value appearing in search results.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Email of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// Handle of the user.
    #[serde(rename = "handle")]
    pub handle: Option<String>,
    /// Name of the user.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// ID of the user.
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentSearchResponseUserFacetData {
    pub fn new() -> IncidentSearchResponseUserFacetData {
        IncidentSearchResponseUserFacetData {
            count: None,
            email: None,
            handle: None,
            name: None,
            uuid: None,
            _unparsed: false,
        }
    }

    pub fn count(&mut self, value: i32) -> &mut Self {
        self.count = Some(value);
        self
    }

    pub fn email(&mut self, value: String) -> &mut Self {
        self.email = Some(value);
        self
    }

    pub fn handle(&mut self, value: String) -> &mut Self {
        self.handle = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn uuid(&mut self, value: String) -> &mut Self {
        self.uuid = Some(value);
        self
    }
}

impl Default for IncidentSearchResponseUserFacetData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentSearchResponseUserFacetData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentSearchResponseUserFacetDataVisitor;
        impl<'a> Visitor<'a> for IncidentSearchResponseUserFacetDataVisitor {
            type Value = IncidentSearchResponseUserFacetData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count: Option<i32> = None;
                let mut email: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut name: Option<String> = None;
                let mut uuid: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count" => {
                            if v.is_null() {
                                continue;
                            }
                            count = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            if v.is_null() {
                                continue;
                            }
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uuid" => {
                            if v.is_null() {
                                continue;
                            }
                            uuid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = IncidentSearchResponseUserFacetData {
                    count,
                    email,
                    handle,
                    name,
                    uuid,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentSearchResponseUserFacetDataVisitor)
    }
}
