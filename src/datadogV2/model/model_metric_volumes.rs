// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Possible response objects for a metric's volume.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MetricVolumes {
    MetricDistinctVolume(Box<crate::datadogV2::model::MetricDistinctVolume>),
    MetricIngestedIndexedVolume(Box<crate::datadogV2::model::MetricIngestedIndexedVolume>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for MetricVolumes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricDistinctVolume>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(MetricVolumes::MetricDistinctVolume(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::MetricIngestedIndexedVolume>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(MetricVolumes::MetricIngestedIndexedVolume(_v));
            }
        }

        return Ok(MetricVolumes::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
