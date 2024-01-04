// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AzureUCConfigPostRequestType {
    #[serde(rename = "azure_uc_config_post_request")]
    AZURE_UC_CONFIG_POST_REQUEST,
}

impl ToString for AzureUCConfigPostRequestType {
    fn to_string(&self) -> String {
        match self {
            Self::AZURE_UC_CONFIG_POST_REQUEST => String::from("azure_uc_config_post_request"),
        }
    }
}