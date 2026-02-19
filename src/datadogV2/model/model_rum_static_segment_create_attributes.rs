// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new static segment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumStaticSegmentCreateAttributes {
    /// A description of the static segment.
    #[serde(rename = "description")]
    pub description: String,
    /// The journey query object used to compute the static segment user list.
    #[serde(rename = "journey_query_object")]
    pub journey_query_object: crate::datadogV2::model::RumStaticSegmentJourneyQueryObject,
    /// The name of the static segment.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of tags for the static segment.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumStaticSegmentCreateAttributes {
    pub fn new(
        description: String,
        journey_query_object: crate::datadogV2::model::RumStaticSegmentJourneyQueryObject,
        name: String,
    ) -> RumStaticSegmentCreateAttributes {
        RumStaticSegmentCreateAttributes {
            description,
            journey_query_object,
            name,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for RumStaticSegmentCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumStaticSegmentCreateAttributesVisitor;
        impl<'a> Visitor<'a> for RumStaticSegmentCreateAttributesVisitor {
            type Value = RumStaticSegmentCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut journey_query_object: Option<
                    crate::datadogV2::model::RumStaticSegmentJourneyQueryObject,
                > = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "journey_query_object" => {
                            journey_query_object =
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
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let journey_query_object = journey_query_object
                    .ok_or_else(|| M::Error::missing_field("journey_query_object"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = RumStaticSegmentCreateAttributes {
                    description,
                    journey_query_object,
                    name,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumStaticSegmentCreateAttributesVisitor)
    }
}
