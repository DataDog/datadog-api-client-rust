// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A CloudWatch metric and the Datadog metric names it produces.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSMetricNameFilterPreviewMetric {
    /// The CloudWatch metric name.
    #[serde(rename = "cw_name")]
    pub cw_name: String,
    /// The Datadog metric names produced from this CloudWatch metric.
    #[serde(rename = "dd_names")]
    pub dd_names: Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewDDName>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSMetricNameFilterPreviewMetric {
    pub fn new(
        cw_name: String,
        dd_names: Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewDDName>,
    ) -> AWSMetricNameFilterPreviewMetric {
        AWSMetricNameFilterPreviewMetric {
            cw_name,
            dd_names,
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

impl<'de> Deserialize<'de> for AWSMetricNameFilterPreviewMetric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSMetricNameFilterPreviewMetricVisitor;
        impl<'a> Visitor<'a> for AWSMetricNameFilterPreviewMetricVisitor {
            type Value = AWSMetricNameFilterPreviewMetric;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cw_name: Option<String> = None;
                let mut dd_names: Option<
                    Vec<crate::datadogV2::model::AWSMetricNameFilterPreviewDDName>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cw_name" => {
                            cw_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dd_names" => {
                            dd_names = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cw_name = cw_name.ok_or_else(|| M::Error::missing_field("cw_name"))?;
                let dd_names = dd_names.ok_or_else(|| M::Error::missing_field("dd_names"))?;

                let content = AWSMetricNameFilterPreviewMetric {
                    cw_name,
                    dd_names,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSMetricNameFilterPreviewMetricVisitor)
    }
}
