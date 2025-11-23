// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentDataAttributes {
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "created_by")]
    pub created_by: Option<crate::datadogV2::model::SegmentDataSource>,
    #[serde(rename = "data_query")]
    pub data_query: crate::datadogV2::model::SegmentDataAttributesDataQuery,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "disabled_at")]
    pub disabled_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "disabled_by")]
    pub disabled_by: Option<crate::datadogV2::model::SegmentDataSource>,
    #[serde(rename = "materialization_row_count")]
    pub materialization_row_count: Option<i64>,
    #[serde(rename = "materialized_at")]
    pub materialized_at: Option<String>,
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "modified_by")]
    pub modified_by: Option<crate::datadogV2::model::SegmentDataSource>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    #[serde(rename = "source")]
    pub source: Option<i64>,
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "version")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentDataAttributes {
    pub fn new(
        data_query: crate::datadogV2::model::SegmentDataAttributesDataQuery,
        name: String,
    ) -> SegmentDataAttributes {
        SegmentDataAttributes {
            created_at: None,
            created_by: None,
            data_query,
            description: None,
            disabled_at: None,
            disabled_by: None,
            materialization_row_count: None,
            materialized_at: None,
            modified_at: None,
            modified_by: None,
            name,
            org_id: None,
            source: None,
            tags: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: crate::datadogV2::model::SegmentDataSource) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn disabled_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.disabled_at = Some(value);
        self
    }

    pub fn disabled_by(mut self, value: crate::datadogV2::model::SegmentDataSource) -> Self {
        self.disabled_by = Some(value);
        self
    }

    pub fn materialization_row_count(mut self, value: i64) -> Self {
        self.materialization_row_count = Some(value);
        self
    }

    pub fn materialized_at(mut self, value: String) -> Self {
        self.materialized_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn modified_by(mut self, value: crate::datadogV2::model::SegmentDataSource) -> Self {
        self.modified_by = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn source(mut self, value: i64) -> Self {
        self.source = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for SegmentDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentDataAttributesVisitor;
        impl<'a> Visitor<'a> for SegmentDataAttributesVisitor {
            type Value = SegmentDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<crate::datadogV2::model::SegmentDataSource> = None;
                let mut data_query: Option<
                    crate::datadogV2::model::SegmentDataAttributesDataQuery,
                > = None;
                let mut description: Option<String> = None;
                let mut disabled_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut disabled_by: Option<crate::datadogV2::model::SegmentDataSource> = None;
                let mut materialization_row_count: Option<i64> = None;
                let mut materialized_at: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<crate::datadogV2::model::SegmentDataSource> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut source: Option<i64> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "disabled_at" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "disabled_by" => {
                            if v.is_null() {
                                continue;
                            }
                            disabled_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "materialization_row_count" => {
                            if v.is_null() {
                                continue;
                            }
                            materialization_row_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "materialized_at" => {
                            if v.is_null() {
                                continue;
                            }
                            materialized_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_by =
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
                        "source" => {
                            if v.is_null() {
                                continue;
                            }
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SegmentDataAttributes {
                    created_at,
                    created_by,
                    data_query,
                    description,
                    disabled_at,
                    disabled_by,
                    materialization_row_count,
                    materialized_at,
                    modified_at,
                    modified_by,
                    name,
                    org_id,
                    source,
                    tags,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentDataAttributesVisitor)
    }
}
