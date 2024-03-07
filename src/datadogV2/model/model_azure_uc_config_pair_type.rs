// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AzureUCConfigPairType {
    #[serde(rename = "azure_uc_configs")]
    AZURE_UC_CONFIGS,
}
impl ToString for AzureUCConfigPairType {
    fn to_string(&self) -> String {
        match self {
            Self::AZURE_UC_CONFIGS => String::from("azure_uc_configs"),
        }
    }
}
