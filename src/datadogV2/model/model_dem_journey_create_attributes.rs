// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a DEM journey.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemJourneyCreateAttributes {
    /// An optional human-readable description of the journey.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The RUM definition for a DEM journey.
    #[serde(rename = "journey_rum")]
    pub journey_rum: crate::datadogV2::model::DemJourneyRum,
    /// The name of the DEM journey.
    #[serde(rename = "name")]
    pub name: String,
    /// List of tags associated with a DEM resource.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// List of variants associated with a DEM journey.
    #[serde(rename = "variants")]
    pub variants: Option<Vec<crate::datadogV2::model::DemVariant>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemJourneyCreateAttributes {
    pub fn new(
        journey_rum: crate::datadogV2::model::DemJourneyRum,
        name: String,
        tags: Vec<String>,
    ) -> DemJourneyCreateAttributes {
        DemJourneyCreateAttributes {
            description: None,
            journey_rum,
            name,
            tags,
            variants: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn variants(mut self, value: Vec<crate::datadogV2::model::DemVariant>) -> Self {
        self.variants = Some(value);
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

impl<'de> Deserialize<'de> for DemJourneyCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemJourneyCreateAttributesVisitor;
        impl<'a> Visitor<'a> for DemJourneyCreateAttributesVisitor {
            type Value = DemJourneyCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut journey_rum: Option<crate::datadogV2::model::DemJourneyRum> = None;
                let mut name: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::DemVariant>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "journey_rum" => {
                            journey_rum =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            if v.is_null() {
                                continue;
                            }
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let journey_rum =
                    journey_rum.ok_or_else(|| M::Error::missing_field("journey_rum"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = DemJourneyCreateAttributes {
                    description,
                    journey_rum,
                    name,
                    tags,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemJourneyCreateAttributesVisitor)
    }
}
