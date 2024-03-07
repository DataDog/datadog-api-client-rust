// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MetricBulkConfigureTagsType {
    #[serde(rename = "metric_bulk_configure_tags")]
    BULK_MANAGE_TAGS,
}

impl ToString for MetricBulkConfigureTagsType {
    fn to_string(&self) -> String {
        match self {
            Self::BULK_MANAGE_TAGS => String::from("metric_bulk_configure_tags"),
        }
    }
}
