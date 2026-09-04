// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Structured tag evidence for an influential-tag finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationFindingTag {
    /// Kind of influence a tag has on a series.
    #[serde(rename = "influence_type")]
    pub influence_type: crate::datadogV2::model::TimeseriesAnomalyInvestigationInfluenceType,
    /// Influential tag key.
    #[serde(rename = "key")]
    pub key: String,
    /// Influence rating from 1 through 5.
    #[serde(rename = "rating")]
    pub rating: f64,
    /// Tags grouped with this tag by Variation of Influence synonym analysis.
    #[serde(rename = "synonyms")]
    pub synonyms: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationFindingSynonym>,
    /// Influential values for the tag key.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationFindingTag {
    pub fn new(
        influence_type: crate::datadogV2::model::TimeseriesAnomalyInvestigationInfluenceType,
        key: String,
        rating: f64,
        synonyms: Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationFindingSynonym>,
        values: Vec<String>,
    ) -> TimeseriesAnomalyInvestigationFindingTag {
        TimeseriesAnomalyInvestigationFindingTag {
            influence_type,
            key,
            rating,
            synonyms,
            values,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationFindingTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationFindingTagVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationFindingTagVisitor {
            type Value = TimeseriesAnomalyInvestigationFindingTag;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut influence_type: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationInfluenceType,
                > = None;
                let mut key: Option<String> = None;
                let mut rating: Option<f64> = None;
                let mut synonyms: Option<
                    Vec<crate::datadogV2::model::TimeseriesAnomalyInvestigationFindingSynonym>,
                > = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "influence_type" => {
                            influence_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _influence_type) = influence_type {
                                match _influence_type {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationInfluenceType::UnparsedObject(_influence_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rating" => {
                            rating = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "synonyms" => {
                            synonyms = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let influence_type =
                    influence_type.ok_or_else(|| M::Error::missing_field("influence_type"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let rating = rating.ok_or_else(|| M::Error::missing_field("rating"))?;
                let synonyms = synonyms.ok_or_else(|| M::Error::missing_field("synonyms"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = TimeseriesAnomalyInvestigationFindingTag {
                    influence_type,
                    key,
                    rating,
                    synonyms,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationFindingTagVisitor)
    }
}
