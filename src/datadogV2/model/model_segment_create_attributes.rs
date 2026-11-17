// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the segment to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentCreateAttributes {
    /// Defines who belongs to the segment. It contains one or more blocks, up to 20 in total.
    /// Use `combination` to combine named blocks with a boolean formula such as `A AND NOT B`.
    #[serde(rename = "data_query")]
    pub data_query: crate::datadogV2::model::SegmentDataQuery,
    /// Description of the segment.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Name of the segment. It must be unique in the organization.
    #[serde(rename = "name")]
    pub name: String,
    /// Tags of the segment. Tags cannot contain commas.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentCreateAttributes {
    pub fn new(
        data_query: crate::datadogV2::model::SegmentDataQuery,
        name: String,
    ) -> SegmentCreateAttributes {
        SegmentCreateAttributes {
            data_query,
            description: None,
            name,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for SegmentCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentCreateAttributesVisitor {
            type Value = SegmentCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_query: Option<crate::datadogV2::model::SegmentDataQuery> = None;
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_query" => {
                            data_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_query = data_query.ok_or_else(|| M::Error::missing_field("data_query"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = SegmentCreateAttributes {
                    data_query,
                    description,
                    name,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentCreateAttributesVisitor)
    }
}
