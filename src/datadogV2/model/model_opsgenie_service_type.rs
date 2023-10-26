// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OpsgenieServiceType {
    #[serde(rename = "opsgenie-service")]
    OPSGENIE_SERVICE,
}

impl ToString for OpsgenieServiceType {
    fn to_string(&self) -> String {
        match self {
            Self::OPSGENIE_SERVICE => String::from("opsgenie-service"),
        }
    }
}

impl Default for OpsgenieServiceType {
    fn default() -> OpsgenieServiceType {
        Self::OPSGENIE_SERVICE
    }
}
