// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApmRetentionFilterType {
    #[serde(rename = "apm_retention_filter")]
    apm_retention_filter,
}

impl ToString for ApmRetentionFilterType {
    fn to_string(&self) -> String {
        match self {
            Self::apm_retention_filter => String::from("apm_retention_filter"),
        }
    }
}

impl Default for ApmRetentionFilterType {
    fn default() -> ApmRetentionFilterType {
        Self::apm_retention_filter
    }
}
