// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Optional Warehouse measure that scopes analyzed subjects.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint
{
    /// Complete ordered Warehouse entry-point filter set.
    #[serde(rename = "filters")]
    pub filters: Vec<crate::datadogV2::model::ExperimentsWarehouseExposureFilter>,
    /// Warehouse measure UUID.
    #[serde(rename = "measure_id")]
    pub measure_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint {
    pub fn new(
        filters: Vec<crate::datadogV2::model::ExperimentsWarehouseExposureFilter>,
        measure_id: String,
    ) -> ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint
    {
        ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint {
            filters,
            measure_id,
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

impl<'de> Deserialize<'de>
    for ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointVisitor {
            type Value = ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filters: Option<Vec<crate::datadogV2::model::ExperimentsWarehouseExposureFilter>> = None;
                let mut measure_id: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filters" => {
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "measure_id" => {
                            measure_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let filters = filters.ok_or_else(|| M::Error::missing_field("filters"))?;
                let measure_id = measure_id.ok_or_else(|| M::Error::missing_field("measure_id"))?;

                let content = ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint {
                    filters,
                    measure_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPointVisitor)
    }
}
