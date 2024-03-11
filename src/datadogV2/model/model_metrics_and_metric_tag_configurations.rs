// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Object for a metrics and metric tag configurations.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MetricsAndMetricTagConfigurations {
    Metric(Box<crate::datadogV2::model::Metric>),
    MetricTagConfiguration(Box<crate::datadogV2::model::MetricTagConfiguration>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for MetricsAndMetricTagConfigurations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::Metric>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(MetricsAndMetricTagConfigurations::Metric(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricTagConfiguration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(MetricsAndMetricTagConfigurations::MetricTagConfiguration(
                    _v,
                ));
            }
        }

        return Ok(MetricsAndMetricTagConfigurations::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
