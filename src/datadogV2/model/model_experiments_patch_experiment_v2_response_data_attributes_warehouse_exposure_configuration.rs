// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Warehouse exposure model and settings used to identify experiment assignments.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration {
    /// Optional Warehouse measure that scopes analyzed subjects.
	#[serialize_always]
    #[serde(rename = "entry_point")]
    pub entry_point: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint>,
    /// Warehouse experiment key. Reads can return null for incomplete configuration; configuration writes require a value.
	#[serialize_always]
    #[serde(rename = "experiment_key")]
    pub experiment_key: Option<String>,
    /// ID of the exposure SQL model that provides assignment data.
    #[serde(rename = "exposure_sql_model_id")]
    pub exposure_sql_model_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration {
    pub fn new(
        entry_point: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint>,
        experiment_key: Option<String>,
        exposure_sql_model_id: String,
    ) -> ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration {
        ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration {
            entry_point,
            experiment_key,
            exposure_sql_model_id,
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
    for ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationVisitor;
        impl<'a> Visitor<'a> for ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationVisitor {
            type Value = ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entry_point: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationEntryPoint>> = None;
                let mut experiment_key: Option<Option<String>> = None;
                let mut exposure_sql_model_id: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entry_point" => {
                            entry_point = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "experiment_key" => {
                            experiment_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "exposure_sql_model_id" => {
                            exposure_sql_model_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let entry_point = entry_point.ok_or_else(|| M::Error::missing_field("entry_point"))?;
                let experiment_key = experiment_key.ok_or_else(|| M::Error::missing_field("experiment_key"))?;
                let exposure_sql_model_id = exposure_sql_model_id.ok_or_else(|| M::Error::missing_field("exposure_sql_model_id"))?;

                let content = ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfiguration {
                    entry_point,
                    experiment_key,
                    exposure_sql_model_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsPatchExperimentV2ResponseDataAttributesWarehouseExposureConfigurationVisitor,
        )
    }
}
