// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a general investigation with an explicit time window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeneralInvestigationAttributesWithTimeBounds {
    /// A free-form description of what to investigate, up to 4,096 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// The end of the investigation window, in Unix milliseconds.
    #[serde(rename = "end_time")]
    pub end_time: i64,
    /// The start of the investigation window, in Unix milliseconds.
    #[serde(rename = "start_time")]
    pub start_time: i64,
    /// Tags that scope the investigation.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeneralInvestigationAttributesWithTimeBounds {
    pub fn new(
        description: String,
        end_time: i64,
        start_time: i64,
    ) -> GeneralInvestigationAttributesWithTimeBounds {
        GeneralInvestigationAttributesWithTimeBounds {
            description,
            end_time,
            start_time,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for GeneralInvestigationAttributesWithTimeBounds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeneralInvestigationAttributesWithTimeBoundsVisitor;
        impl<'a> Visitor<'a> for GeneralInvestigationAttributesWithTimeBoundsVisitor {
            type Value = GeneralInvestigationAttributesWithTimeBounds;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut end_time: Option<i64> = None;
                let mut start_time: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_time" => {
                            end_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_time" => {
                            start_time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let end_time = end_time.ok_or_else(|| M::Error::missing_field("end_time"))?;
                let start_time = start_time.ok_or_else(|| M::Error::missing_field("start_time"))?;

                let content = GeneralInvestigationAttributesWithTimeBounds {
                    description,
                    end_time,
                    start_time,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeneralInvestigationAttributesWithTimeBoundsVisitor)
    }
}
