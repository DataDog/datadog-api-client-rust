// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfluentAccountType {
    #[serde(rename = "confluent-cloud-accounts")]
    CONFLUENT_CLOUD_ACCOUNTS,
}

impl ToString for ConfluentAccountType {
    fn to_string(&self) -> String {
        match self {
            Self::CONFLUENT_CLOUD_ACCOUNTS => String::from("confluent-cloud-accounts"),
        }
    }
}

impl Default for ConfluentAccountType {
    fn default() -> ConfluentAccountType {
        Self::CONFLUENT_CLOUD_ACCOUNTS
    }
}
