// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GCPServiceAccountType {
    #[serde(rename = "gcp_service_account")]
    GCP_SERVICE_ACCOUNT,
}

impl ToString for GCPServiceAccountType {
    fn to_string(&self) -> String {
        match self {
            Self::GCP_SERVICE_ACCOUNT => String::from("gcp_service_account"),
        }
    }
}

impl Default for GCPServiceAccountType {
    fn default() -> GCPServiceAccountType {
        Self::GCP_SERVICE_ACCOUNT
    }
}