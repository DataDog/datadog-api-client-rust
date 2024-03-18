// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A service level objective ID and attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchServiceLevelObjectiveData {
    /// A service level objective object includes a service level indicator, thresholds
    /// for one or more timeframes, and metadata (`name`, `description`, and `tags`).
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::SearchServiceLevelObjectiveAttributes>,
    /// A unique identifier for the service level objective object.
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the object, must be `slo`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchServiceLevelObjectiveData {
    pub fn new() -> SearchServiceLevelObjectiveData {
        SearchServiceLevelObjectiveData {
            attributes: None,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::SearchServiceLevelObjectiveAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SearchServiceLevelObjectiveData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchServiceLevelObjectiveData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchServiceLevelObjectiveDataVisitor;
        impl<'a> Visitor<'a> for SearchServiceLevelObjectiveDataVisitor {
            type Value = SearchServiceLevelObjectiveData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::SearchServiceLevelObjectiveAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SearchServiceLevelObjectiveData {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchServiceLevelObjectiveDataVisitor)
    }
}
