// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Object for a metrics and metric tag configurations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricsAndMetricTagConfigurations {
    Metric(Box<crate::datadogV2::model::Metric>),
    MetricTagConfiguration(Box<crate::datadogV2::model::MetricTagConfiguration>),
}