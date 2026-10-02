// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Model entries removed by the update. Empty arrays mean no entries were removed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsUpdateMetricSQLModelV2ResponseMeta {
    /// Measure column names removed from the model.
    #[serde(rename = "deleted_measures")]
    pub deleted_measures: Option<Vec<String>>,
    /// Property names removed from the model.
    #[serde(rename = "deleted_properties")]
    pub deleted_properties: Option<Vec<String>>,
    /// Subject type IDs removed from the model.
    #[serde(rename = "deleted_subject_types")]
    pub deleted_subject_types: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsUpdateMetricSQLModelV2ResponseMeta {
    pub fn new() -> ExperimentsUpdateMetricSQLModelV2ResponseMeta {
        ExperimentsUpdateMetricSQLModelV2ResponseMeta {
            deleted_measures: None,
            deleted_properties: None,
            deleted_subject_types: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deleted_measures(mut self, value: Vec<String>) -> Self {
        self.deleted_measures = Some(value);
        self
    }

    pub fn deleted_properties(mut self, value: Vec<String>) -> Self {
        self.deleted_properties = Some(value);
        self
    }

    pub fn deleted_subject_types(mut self, value: Vec<String>) -> Self {
        self.deleted_subject_types = Some(value);
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

impl Default for ExperimentsUpdateMetricSQLModelV2ResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsUpdateMetricSQLModelV2ResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsUpdateMetricSQLModelV2ResponseMetaVisitor;
        impl<'a> Visitor<'a> for ExperimentsUpdateMetricSQLModelV2ResponseMetaVisitor {
            type Value = ExperimentsUpdateMetricSQLModelV2ResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_measures: Option<Vec<String>> = None;
                let mut deleted_properties: Option<Vec<String>> = None;
                let mut deleted_subject_types: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_measures" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_measures =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_properties" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_subject_types" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_subject_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsUpdateMetricSQLModelV2ResponseMeta {
                    deleted_measures,
                    deleted_properties,
                    deleted_subject_types,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsUpdateMetricSQLModelV2ResponseMetaVisitor)
    }
}
