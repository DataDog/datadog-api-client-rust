// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a RUM operation response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMOperationResponseAttributes {
    /// The RUM application ID the operation belongs to.
    #[serde(
        rename = "application_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub application_id: Option<Option<uuid::Uuid>>,
    /// The category of the RUM operation.
    #[serde(
        rename = "category",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub category: Option<Option<String>>,
    /// The timestamp when the RUM operation was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A Datadog user referenced by a RUM operation.
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::RUMOperationUser>,
    /// A description of the RUM operation.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// A human-readable display name for the RUM operation.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// The list of feature IDs associated with the RUM operation.
    #[serde(rename = "feature_ids")]
    pub feature_ids: Option<Vec<String>>,
    /// The definition of a RUM operation's journey, used to detect it from RUM events.
    #[serde(rename = "journey_rum")]
    pub journey_rum: crate::datadogV2::model::RUMOperationJourneyRum,
    /// The unique name of the RUM operation. Must not contain spaces.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the organization the RUM operation belongs to.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// A list of tags associated with the RUM operation.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// The timestamp when the RUM operation was last updated.
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub updated_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A Datadog user referenced by a RUM operation.
    #[serde(rename = "updated_by")]
    pub updated_by: Option<crate::datadogV2::model::RUMOperationUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMOperationResponseAttributes {
    pub fn new(
        journey_rum: crate::datadogV2::model::RUMOperationJourneyRum,
        name: String,
        tags: Vec<String>,
    ) -> RUMOperationResponseAttributes {
        RUMOperationResponseAttributes {
            application_id: None,
            category: None,
            created_at: None,
            created_by: None,
            description: None,
            display_name: None,
            feature_ids: None,
            journey_rum,
            name,
            org_id: None,
            tags,
            updated_at: None,
            updated_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: Option<uuid::Uuid>) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn category(mut self, value: Option<String>) -> Self {
        self.category = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: crate::datadogV2::model::RUMOperationUser) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn feature_ids(mut self, value: Vec<String>) -> Self {
        self.feature_ids = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn updated_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by(mut self, value: crate::datadogV2::model::RUMOperationUser) -> Self {
        self.updated_by = Some(value);
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

impl<'de> Deserialize<'de> for RUMOperationResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMOperationResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RUMOperationResponseAttributesVisitor {
            type Value = RUMOperationResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<Option<uuid::Uuid>> = None;
                let mut category: Option<Option<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::RUMOperationUser> = None;
                let mut description: Option<Option<String>> = None;
                let mut display_name: Option<String> = None;
                let mut feature_ids: Option<Vec<String>> = None;
                let mut journey_rum: Option<crate::datadogV2::model::RUMOperationJourneyRum> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut updated_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut updated_by: Option<crate::datadogV2::model::RUMOperationUser> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            feature_ids =
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
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = RUMOperationResponseAttributes {
                    application_id,
                    category,
                    created_at,
                    created_by,
                    description,
                    display_name,
                    feature_ids,
                    journey_rum,
                    name,
                    org_id,
                    tags,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMOperationResponseAttributesVisitor)
    }
}
