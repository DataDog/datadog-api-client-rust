// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// List of included assets with full set of attributes.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MetricAssetResponseIncluded {
    MetricDashboardAsset(Box<crate::datadogV2::model::MetricDashboardAsset>),
    MetricMonitorAsset(Box<crate::datadogV2::model::MetricMonitorAsset>),
    MetricNotebookAsset(Box<crate::datadogV2::model::MetricNotebookAsset>),
    MetricSLOAsset(Box<crate::datadogV2::model::MetricSLOAsset>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for MetricAssetResponseIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricDashboardAsset>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(MetricAssetResponseIncluded::MetricDashboardAsset(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricMonitorAsset>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(MetricAssetResponseIncluded::MetricMonitorAsset(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MetricNotebookAsset>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(MetricAssetResponseIncluded::MetricNotebookAsset(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::MetricSLOAsset>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(MetricAssetResponseIncluded::MetricSLOAsset(_v));
            }
        }

        return Ok(MetricAssetResponseIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
