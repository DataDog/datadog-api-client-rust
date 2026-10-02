// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A property comparison for metric source data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPropertyFilterInput {
    /// Omit this target or use null or a blank string.
    #[serde(rename = "measure_id", default, with = "::serde_with::rust::double_option")]
    pub measure_id: Option<Option<String>>,
    /// Comparison applied by the warehouse entry-point filter.
    #[serde(rename = "operation")]
    pub operation: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointFiltersItemsOperation,
    /// ID of the property on the aggregation source.
    #[serde(rename = "property_id")]
    pub property_id: uuid::Uuid,
    /// Values used by the comparison.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPropertyFilterInput {
    pub fn new(
        operation: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointFiltersItemsOperation,
        property_id: uuid::Uuid,
        values: Vec<String>,
    ) -> ExperimentsPropertyFilterInput {
        ExperimentsPropertyFilterInput {
            measure_id: None,
            operation,
            property_id,
            values,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn measure_id(mut self, value: Option<String>) -> Self {
        self.measure_id = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsPropertyFilterInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPropertyFilterInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsPropertyFilterInputVisitor {
            type Value = ExperimentsPropertyFilterInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut measure_id: Option<Option<String>> = None;
                let mut operation: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointFiltersItemsOperation> = None;
                let mut property_id: Option<uuid::Uuid> = None;
                let mut values: Option<Vec<String>> = None;
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
                                    crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointFiltersItemsOperation::UnparsedObject(_operation) => {
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
                let operation = operation.ok_or_else(|| M::Error::missing_field("operation"))?;
                let property_id =
                    property_id.ok_or_else(|| M::Error::missing_field("property_id"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = ExperimentsPropertyFilterInput {
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

        deserializer.deserialize_any(ExperimentsPropertyFilterInputVisitor)
    }
}
