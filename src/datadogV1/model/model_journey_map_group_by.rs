// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Group by configuration for the Journey Paths widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JourneyMapGroupBy {
    /// Facet to group by. Must be a valid tag name.
    #[serde(rename = "facet")]
    pub facet: String,
    /// Audience the group by facet is read from.
    #[serde(rename = "source")]
    pub source: Option<crate::datadogV1::model::JourneyMapGroupByAudience>,
    /// Facet values to keep.
    #[serde(rename = "value_filters")]
    pub value_filters: Vec<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JourneyMapGroupBy {
    pub fn new(facet: String, value_filters: Vec<String>) -> JourneyMapGroupBy {
        JourneyMapGroupBy {
            facet,
            source: None,
            value_filters,
            _unparsed: false,
        }
    }

    pub fn source(mut self, value: crate::datadogV1::model::JourneyMapGroupByAudience) -> Self {
        self.source = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for JourneyMapGroupBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JourneyMapGroupByVisitor;
        impl<'a> Visitor<'a> for JourneyMapGroupByVisitor {
            type Value = JourneyMapGroupBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut facet: Option<String> = None;
                let mut source: Option<crate::datadogV1::model::JourneyMapGroupByAudience> = None;
                let mut value_filters: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "facet" => {
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV1::model::JourneyMapGroupByAudience::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value_filters" => {
                            value_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let facet = facet.ok_or_else(|| M::Error::missing_field("facet"))?;
                let value_filters =
                    value_filters.ok_or_else(|| M::Error::missing_field("value_filters"))?;

                let content = JourneyMapGroupBy {
                    facet,
                    source,
                    value_filters,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JourneyMapGroupByVisitor)
    }
}
