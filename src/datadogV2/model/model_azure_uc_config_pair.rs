// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Azure config pair.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureUCConfigPair {
    /// Attributes for Azure config pair.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::AzureUCConfigPairAttributes>,
    /// The ID of Cloud Cost Management account.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Type of Azure config pair.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AzureUCConfigPairType,
}

impl AzureUCConfigPair {
    pub fn new(
        attributes: Box<crate::datadogV2::model::AzureUCConfigPairAttributes>,
        type_: crate::datadogV2::model::AzureUCConfigPairType,
    ) -> AzureUCConfigPair {
        AzureUCConfigPair {
            attributes,
            id: None,
            type_,
        }
    }
}
