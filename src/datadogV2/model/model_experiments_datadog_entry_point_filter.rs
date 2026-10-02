// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Complete Datadog OR-of-ANDs entry-point filter expression.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsDatadogEntryPointFilter {
    /// Exposure field evaluated by the entry-point filter.
    #[serde(rename = "column")]
    pub column: String,
    /// Data type of the column evaluated by the entry-point filter.
    #[serde(rename = "column_type")]
    pub column_type: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsColumnType,
    /// Comparison applied by the Datadog entry-point filter.
    #[serde(rename = "operation")]
    pub operation: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation,
    /// Comparison values used by the filter operation.
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsDatadogEntryPointFilter {
    pub fn new(
        column: String,
        column_type: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsColumnType,
        operation: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation,
        values: Vec<String>,
    ) -> ExperimentsDatadogEntryPointFilter {
        ExperimentsDatadogEntryPointFilter {
            column,
            column_type,
            operation,
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

impl<'de> Deserialize<'de> for ExperimentsDatadogEntryPointFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsDatadogEntryPointFilterVisitor;
        impl<'a> Visitor<'a> for ExperimentsDatadogEntryPointFilterVisitor {
            type Value = ExperimentsDatadogEntryPointFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column: Option<String> = None;
                let mut column_type: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsColumnType> = None;
                let mut operation: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column" => {
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column_type" => {
                            column_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _column_type) = column_type {
                                match _column_type {
                                    crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsColumnType::UnparsedObject(_column_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "operation" => {
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operation) = operation {
                                match _operation {
                                    crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPointFiltersItemsItemsOperation::UnparsedObject(_operation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                let column = column.ok_or_else(|| M::Error::missing_field("column"))?;
                let column_type =
                    column_type.ok_or_else(|| M::Error::missing_field("column_type"))?;
                let operation = operation.ok_or_else(|| M::Error::missing_field("operation"))?;
                let values = values.ok_or_else(|| M::Error::missing_field("values"))?;

                let content = ExperimentsDatadogEntryPointFilter {
                    column,
                    column_type,
                    operation,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsDatadogEntryPointFilterVisitor)
    }
}
