// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Additional configuration options for snapshot creation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateSnapshotAdditionalConfig {
    /// List of template variable definitions for snapshot rendering.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<Vec<crate::datadogV2::model::CreateSnapshotTemplateVariable>>,
    /// The legend display type for timeseries widgets. A value of `none` hides the legend entirely; omitting the field lets the frontend choose automatically.
    #[serde(rename = "timeseries_legend_type")]
    pub timeseries_legend_type: Option<crate::datadogV2::model::CreateSnapshotTimeseriesLegendType>,
    /// Timezone offset in minutes from UTC. Positive values are west of UTC (for example, `300` for UTC-5). Use `0` for UTC.
    #[serde(rename = "timezone_offset_minutes")]
    pub timezone_offset_minutes: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateSnapshotAdditionalConfig {
    pub fn new() -> CreateSnapshotAdditionalConfig {
        CreateSnapshotAdditionalConfig {
            template_variables: None,
            timeseries_legend_type: None,
            timezone_offset_minutes: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn template_variables(
        mut self,
        value: Vec<crate::datadogV2::model::CreateSnapshotTemplateVariable>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }

    pub fn timeseries_legend_type(
        mut self,
        value: crate::datadogV2::model::CreateSnapshotTimeseriesLegendType,
    ) -> Self {
        self.timeseries_legend_type = Some(value);
        self
    }

    pub fn timezone_offset_minutes(mut self, value: i64) -> Self {
        self.timezone_offset_minutes = Some(value);
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

impl Default for CreateSnapshotAdditionalConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CreateSnapshotAdditionalConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateSnapshotAdditionalConfigVisitor;
        impl<'a> Visitor<'a> for CreateSnapshotAdditionalConfigVisitor {
            type Value = CreateSnapshotAdditionalConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::CreateSnapshotTemplateVariable>,
                > = None;
                let mut timeseries_legend_type: Option<
                    crate::datadogV2::model::CreateSnapshotTimeseriesLegendType,
                > = None;
                let mut timezone_offset_minutes: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "template_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeseries_legend_type" => {
                            if v.is_null() {
                                continue;
                            }
                            timeseries_legend_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timeseries_legend_type) = timeseries_legend_type {
                                match _timeseries_legend_type {
                                    crate::datadogV2::model::CreateSnapshotTimeseriesLegendType::UnparsedObject(_timeseries_legend_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "timezone_offset_minutes" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone_offset_minutes =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CreateSnapshotAdditionalConfig {
                    template_variables,
                    timeseries_legend_type,
                    timezone_offset_minutes,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateSnapshotAdditionalConfigVisitor)
    }
}
