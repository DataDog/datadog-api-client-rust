// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing a search issue request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssuesSearchRequestDataAttributes {
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// Array of indexes to query.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Persona for the search.
    #[serde(rename = "persona")]
    pub persona: Option<crate::datadogV2::model::IssuesSearchRequestDataAttributesPersona>,
    /// Search query following the event search syntax.
    #[serde(rename = "query")]
    pub query: String,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
    /// Track of the events to query.
    #[serde(rename = "track")]
    pub track: Option<crate::datadogV2::model::IssuesSearchRequestDataAttributesTrack>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssuesSearchRequestDataAttributes {
    pub fn new(from: i64, query: String, to: i64) -> IssuesSearchRequestDataAttributes {
        IssuesSearchRequestDataAttributes {
            from,
            indexes: None,
            persona: None,
            query,
            to,
            track: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn persona(
        mut self,
        value: crate::datadogV2::model::IssuesSearchRequestDataAttributesPersona,
    ) -> Self {
        self.persona = Some(value);
        self
    }

    pub fn track(
        mut self,
        value: crate::datadogV2::model::IssuesSearchRequestDataAttributesTrack,
    ) -> Self {
        self.track = Some(value);
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

impl<'de> Deserialize<'de> for IssuesSearchRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssuesSearchRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for IssuesSearchRequestDataAttributesVisitor {
            type Value = IssuesSearchRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<i64> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut persona: Option<
                    crate::datadogV2::model::IssuesSearchRequestDataAttributesPersona,
                > = None;
                let mut query: Option<String> = None;
                let mut to: Option<i64> = None;
                let mut track: Option<
                    crate::datadogV2::model::IssuesSearchRequestDataAttributesTrack,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "persona" => {
                            if v.is_null() {
                                continue;
                            }
                            persona = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _persona) = persona {
                                match _persona {
                                    crate::datadogV2::model::IssuesSearchRequestDataAttributesPersona::UnparsedObject(_persona) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "track" => {
                            if v.is_null() {
                                continue;
                            }
                            track = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _track) = track {
                                match _track {
                                    crate::datadogV2::model::IssuesSearchRequestDataAttributesTrack::UnparsedObject(_track) => {
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
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = IssuesSearchRequestDataAttributes {
                    from,
                    indexes,
                    persona,
                    query,
                    to,
                    track,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssuesSearchRequestDataAttributesVisitor)
    }
}
