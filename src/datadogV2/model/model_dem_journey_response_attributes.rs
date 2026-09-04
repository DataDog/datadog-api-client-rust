// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes returned in a DEM journey response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemJourneyResponseAttributes {
    /// The timestamp when the journey was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A Datadog user associated with a DEM operation.
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::DemUser,
    /// An optional human-readable description of the journey.
    #[serialize_always]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The RUM definition for a DEM journey.
    #[serde(rename = "journey_rum")]
    pub journey_rum: crate::datadogV2::model::DemJourneyRum,
    /// The name of the DEM journey.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization ID that owns this journey.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// List of tags associated with a DEM resource.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// A test suite associated with a DEM resource.
    #[serde(rename = "test_suite")]
    pub test_suite: crate::datadogV2::model::DemTestSuiteNested,
    /// The timestamp when the journey was last updated.
    #[serialize_always]
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A Datadog user associated with a DEM operation.
    #[serde(rename = "updated_by")]
    pub updated_by: crate::datadogV2::model::DemUser,
    /// List of variants associated with a DEM journey.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::DemVariant>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemJourneyResponseAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: crate::datadogV2::model::DemUser,
        description: Option<String>,
        journey_rum: crate::datadogV2::model::DemJourneyRum,
        name: String,
        org_id: i64,
        tags: Vec<String>,
        test_suite: crate::datadogV2::model::DemTestSuiteNested,
        updated_at: Option<chrono::DateTime<chrono::Utc>>,
        updated_by: crate::datadogV2::model::DemUser,
        variants: Vec<crate::datadogV2::model::DemVariant>,
    ) -> DemJourneyResponseAttributes {
        DemJourneyResponseAttributes {
            created_at,
            created_by,
            description,
            journey_rum,
            name,
            org_id,
            tags,
            test_suite,
            updated_at,
            updated_by,
            variants,
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

impl<'de> Deserialize<'de> for DemJourneyResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemJourneyResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DemJourneyResponseAttributesVisitor {
            type Value = DemJourneyResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::DemUser> = None;
                let mut description: Option<Option<String>> = None;
                let mut journey_rum: Option<crate::datadogV2::model::DemJourneyRum> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut test_suite: Option<crate::datadogV2::model::DemTestSuiteNested> = None;
                let mut updated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut updated_by: Option<crate::datadogV2::model::DemUser> = None;
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
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_suite" => {
                            test_suite = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let journey_rum =
                    journey_rum.ok_or_else(|| M::Error::missing_field("journey_rum"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;
                let test_suite = test_suite.ok_or_else(|| M::Error::missing_field("test_suite"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = DemJourneyResponseAttributes {
                    created_at,
                    created_by,
                    description,
                    journey_rum,
                    name,
                    org_id,
                    tags,
                    test_suite,
                    updated_at,
                    updated_by,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemJourneyResponseAttributesVisitor)
    }
}
