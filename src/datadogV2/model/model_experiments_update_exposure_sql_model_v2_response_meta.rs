// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Removed model entries. Present only when the update removes an entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsUpdateExposureSQLModelV2ResponseMeta {
    /// Property names removed from the model.
    #[serde(rename = "removed_property_names")]
    pub removed_property_names: Option<Vec<String>>,
    /// Subject type IDs removed from the model.
    #[serde(rename = "removed_subject_type_ids")]
    pub removed_subject_type_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsUpdateExposureSQLModelV2ResponseMeta {
    pub fn new() -> ExperimentsUpdateExposureSQLModelV2ResponseMeta {
        ExperimentsUpdateExposureSQLModelV2ResponseMeta {
            removed_property_names: None,
            removed_subject_type_ids: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn removed_property_names(mut self, value: Vec<String>) -> Self {
        self.removed_property_names = Some(value);
        self
    }

    pub fn removed_subject_type_ids(mut self, value: Vec<String>) -> Self {
        self.removed_subject_type_ids = Some(value);
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

impl Default for ExperimentsUpdateExposureSQLModelV2ResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsUpdateExposureSQLModelV2ResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsUpdateExposureSQLModelV2ResponseMetaVisitor;
        impl<'a> Visitor<'a> for ExperimentsUpdateExposureSQLModelV2ResponseMetaVisitor {
            type Value = ExperimentsUpdateExposureSQLModelV2ResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut removed_property_names: Option<Vec<String>> = None;
                let mut removed_subject_type_ids: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "removed_property_names" => {
                            if v.is_null() {
                                continue;
                            }
                            removed_property_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "removed_subject_type_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            removed_subject_type_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsUpdateExposureSQLModelV2ResponseMeta {
                    removed_property_names,
                    removed_subject_type_ids,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsUpdateExposureSQLModelV2ResponseMetaVisitor)
    }
}
