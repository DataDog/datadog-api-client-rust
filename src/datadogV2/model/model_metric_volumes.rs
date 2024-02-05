// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Possible response objects for a metric's volume.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricVolumes {
    MetricDistinctVolume(Box<crate::datadogV2::model::MetricDistinctVolume>),
    MetricIngestedIndexedVolume(Box<crate::datadogV2::model::MetricIngestedIndexedVolume>),
}
