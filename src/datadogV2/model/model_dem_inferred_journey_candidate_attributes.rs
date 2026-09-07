// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an inferred journey candidate.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemInferredJourneyCandidateAttributes {
    /// Timestamp when the inferred journey was first observed.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// An optional description of the inferred journey.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The RUM definition for a DEM journey.
    #[serde(rename = "journey_rum")]
    pub journey_rum: crate::datadogV2::model::DemJourneyRum,
    /// The name of the inferred journey.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization ID that owns this inferred journey.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Ranking score of the inferred journey candidate.
    #[serde(rename = "rank", default, with = "::serde_with::rust::double_option")]
    pub rank: Option<Option<i64>>,
    /// List of tags associated with a DEM resource.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// A test suite associated with a DEM resource.
    #[serde(rename = "test_suite")]
    pub test_suite: Option<crate::datadogV2::model::DemTestSuiteNested>,
    /// List of variants associated with a DEM journey.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::DemVariant>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemInferredJourneyCandidateAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        journey_rum: crate::datadogV2::model::DemJourneyRum,
        name: String,
        org_id: i64,
        tags: Vec<String>,
        variants: Vec<crate::datadogV2::model::DemVariant>,
    ) -> DemInferredJourneyCandidateAttributes {
        DemInferredJourneyCandidateAttributes {
            created_at,
            description: None,
            journey_rum,
            name,
            org_id,
            rank: None,
            tags,
            test_suite: None,
            variants,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn rank(mut self, value: Option<i64>) -> Self {
        self.rank = Some(value);
        self
    }

    pub fn test_suite(mut self, value: crate::datadogV2::model::DemTestSuiteNested) -> Self {
        self.test_suite = Some(value);
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

impl<'de> Deserialize<'de> for DemInferredJourneyCandidateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemInferredJourneyCandidateAttributesVisitor;
        impl<'a> Visitor<'a> for DemInferredJourneyCandidateAttributesVisitor {
            type Value = DemInferredJourneyCandidateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<Option<String>> = None;
                let mut journey_rum: Option<crate::datadogV2::model::DemJourneyRum> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut rank: Option<Option<i64>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut test_suite: Option<crate::datadogV2::model::DemTestSuiteNested> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::DemVariant>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
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
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rank" => {
                            rank = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_suite" => {
                            if v.is_null() {
                                continue;
                            }
                            test_suite = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let journey_rum =
                    journey_rum.ok_or_else(|| M::Error::missing_field("journey_rum"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = DemInferredJourneyCandidateAttributes {
                    created_at,
                    description,
                    journey_rum,
                    name,
                    org_id,
                    rank,
                    tags,
                    test_suite,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemInferredJourneyCandidateAttributesVisitor)
    }
}
