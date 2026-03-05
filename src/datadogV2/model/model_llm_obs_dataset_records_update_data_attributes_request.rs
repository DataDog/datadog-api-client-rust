// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating records in an LLM Observability dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsDatasetRecordsUpdateDataAttributesRequest {
    /// List of records to update.
    #[serde(rename = "records")]
    pub records: Vec<crate::datadogV2::model::LLMObsDatasetRecordUpdateItem>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsDatasetRecordsUpdateDataAttributesRequest {
    pub fn new(
        records: Vec<crate::datadogV2::model::LLMObsDatasetRecordUpdateItem>,
    ) -> LLMObsDatasetRecordsUpdateDataAttributesRequest {
        LLMObsDatasetRecordsUpdateDataAttributesRequest {
            records,
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

impl<'de> Deserialize<'de> for LLMObsDatasetRecordsUpdateDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsDatasetRecordsUpdateDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for LLMObsDatasetRecordsUpdateDataAttributesRequestVisitor {
            type Value = LLMObsDatasetRecordsUpdateDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut records: Option<
                    Vec<crate::datadogV2::model::LLMObsDatasetRecordUpdateItem>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "records" => {
                            records = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let records = records.ok_or_else(|| M::Error::missing_field("records"))?;

                let content = LLMObsDatasetRecordsUpdateDataAttributesRequest {
                    records,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsDatasetRecordsUpdateDataAttributesRequestVisitor)
    }
}
