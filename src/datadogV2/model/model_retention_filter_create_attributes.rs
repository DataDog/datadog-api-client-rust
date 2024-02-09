// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing the configuration of the retention filter to create/update.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterCreateAttributes {
    /// Enable/Disable the retention filter.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The spans filter. Spans matching this filter will be indexed and stored.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV2::model::SpansFilterCreate,
    /// The type of retention filter. The value should always be spans-sampling-processor.
    #[serde(rename = "filter_type")]
    pub filter_type: crate::datadogV2::model::RetentionFilterType,
    /// The name of the retention filter.
    #[serde(rename = "name")]
    pub name: String,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    #[serde(rename = "rate")]
    pub rate: f64,
}

impl RetentionFilterCreateAttributes {
    pub fn new(
        enabled: bool,
        filter: crate::datadogV2::model::SpansFilterCreate,
        filter_type: crate::datadogV2::model::RetentionFilterType,
        name: String,
        rate: f64,
    ) -> RetentionFilterCreateAttributes {
        RetentionFilterCreateAttributes {
            enabled,
            filter,
            filter_type,
            name,
            rate,
        }
    }
}
