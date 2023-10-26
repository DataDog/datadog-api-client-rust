// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GCPSTSDelegateAccountType {
    #[serde(rename = "gcp_sts_delegate")]
    GCP_STS_DELEGATE,
}

impl ToString for GCPSTSDelegateAccountType {
    fn to_string(&self) -> String {
        match self {
            Self::GCP_STS_DELEGATE => String::from("gcp_sts_delegate"),
        }
    }
}

impl Default for GCPSTSDelegateAccountType {
    fn default() -> GCPSTSDelegateAccountType {
        Self::GCP_STS_DELEGATE
    }
}
