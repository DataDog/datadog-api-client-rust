// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A measure comparison for metric source data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsMeasureNullFilterInput {
    /// ID of the measure on the aggregation source.
    #[serde(rename = "measure_id")]
    pub measure_id: uuid::Uuid,
    /// Comparison applied by this filter.
    #[serde(rename = "operation")]
    pub operation: crate::datadogV2::model::ExperimentsPropertyNullFilterInputOperation,
    /// Omit this target or use null or a blank string.
    #[serde(
        rename = "property_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub property_id: Option<Option<String>>,
    /// Values used by the comparison.
    #[serde(rename = "values", default, with = "::serde_with::rust::double_option")]
    pub values: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsMeasureNullFilterInput {
    pub fn new(
        measure_id: uuid::Uuid,
        operation: crate::datadogV2::model::ExperimentsPropertyNullFilterInputOperation,
    ) -> ExperimentsMeasureNullFilterInput {
        ExperimentsMeasureNullFilterInput {
            measure_id,
            operation,
            property_id: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn property_id(mut self, value: Option<String>) -> Self {
        self.property_id = Some(value);
        self
    }

    pub fn values(mut self, value: Option<Vec<String>>) -> Self {
        self.values = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsMeasureNullFilterInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsMeasureNullFilterInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsMeasureNullFilterInputVisitor {
            type Value = ExperimentsMeasureNullFilterInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut measure_id: Option<uuid::Uuid> = None;
                let mut operation: Option<
                    crate::datadogV2::model::ExperimentsPropertyNullFilterInputOperation,
                > = None;
                let mut property_id: Option<Option<String>> = None;
                let mut values: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "measure_id" => {
                            measure_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation" => {
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operation) = operation {
                                match _operation {
                                    crate::datadogV2::model::ExperimentsPropertyNullFilterInputOperation::UnparsedObject(_operation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "property_id" => {
                            property_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let measure_id = measure_id.ok_or_else(|| M::Error::missing_field("measure_id"))?;
                let operation = operation.ok_or_else(|| M::Error::missing_field("operation"))?;

                let content = ExperimentsMeasureNullFilterInput {
                    measure_id,
                    operation,
                    property_id,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsMeasureNullFilterInputVisitor)
    }
}
