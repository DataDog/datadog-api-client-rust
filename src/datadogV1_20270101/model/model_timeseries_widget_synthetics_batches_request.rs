// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesWidgetSyntheticsBatchesRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "display_type")]
    pub display_type: Option<crate::datadogV1_20270101::model::WidgetDisplayType>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "metadata")]
    pub metadata: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "on_right_yaxis")]
    pub on_right_yaxis: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::TimeseriesRequestStyle>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "synthetics_batches")]
    pub synthetics_batches: crate::datadogV1_20270101::model::LogQueryDefinition,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TimeseriesWidgetSyntheticsBatchesRequest {
    pub fn new(synthetics_batches: crate::datadogV1_20270101::model::LogQueryDefinition) -> TimeseriesWidgetSyntheticsBatchesRequest {
        TimeseriesWidgetSyntheticsBatchesRequest {
            display_type: None,
            metadata: None,
            on_right_yaxis: None,
            style: None,
            synthetics_batches,
            _unparsed: false,
        }
    }
    
    pub fn display_type(mut self, value: crate::datadogV1_20270101::model::WidgetDisplayType) -> Self {
        self.display_type = Some(value);
        self
    }
    
    pub fn metadata(mut self, value: Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>) -> Self {
        self.metadata = Some(value);
        self
    }
    
    pub fn on_right_yaxis(mut self, value: bool) -> Self {
        self.on_right_yaxis = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::TimeseriesRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TimeseriesWidgetSyntheticsBatchesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesWidgetSyntheticsBatchesRequestVisitor;
        impl<'a> Visitor<'a> for TimeseriesWidgetSyntheticsBatchesRequestVisitor {
            type Value = TimeseriesWidgetSyntheticsBatchesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_type: Option<crate::datadogV1_20270101::model::WidgetDisplayType> = None;
                let mut metadata: Option<Vec<crate::datadogV1_20270101::model::TimeseriesWidgetExpressionAlias>> = None;
                let mut on_right_yaxis: Option<bool> = None;
                let mut style: Option<crate::datadogV1_20270101::model::TimeseriesRequestStyle> = None;
                let mut synthetics_batches: Option<crate::datadogV1_20270101::model::LogQueryDefinition> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_type" => {
                            if v.is_null() {
                                continue;
                            }
                            display_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _display_type) = display_type {
                                match _display_type {
                                    crate::datadogV1_20270101::model::WidgetDisplayType::UnparsedObject(_display_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "on_right_yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            on_right_yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "synthetics_batches" => {
                            synthetics_batches = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let synthetics_batches = synthetics_batches.ok_or_else(|| M::Error::missing_field("synthetics_batches"))?;

                let content = TimeseriesWidgetSyntheticsBatchesRequest {
                    display_type,
                    metadata,
                    on_right_yaxis,
                    style,
                    synthetics_batches,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesWidgetSyntheticsBatchesRequestVisitor)
    }
}
