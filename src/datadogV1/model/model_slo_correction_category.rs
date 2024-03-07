// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SLOCorrectionCategory {
    #[serde(rename = "Scheduled Maintenance")]
    SCHEDULED_MAINTENANCE,
    #[serde(rename = "Outside Business Hours")]
    OUTSIDE_BUSINESS_HOURS,
    #[serde(rename = "Deployment")]
    DEPLOYMENT,
    #[serde(rename = "Other")]
    OTHER,
}

impl ToString for SLOCorrectionCategory {
    fn to_string(&self) -> String {
        match self {
            Self::SCHEDULED_MAINTENANCE => String::from("Scheduled Maintenance"),
            Self::OUTSIDE_BUSINESS_HOURS => String::from("Outside Business Hours"),
            Self::DEPLOYMENT => String::from("Deployment"),
            Self::OTHER => String::from("Other"),
        }
    }
}
