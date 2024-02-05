// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for Azure config pair.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureUCConfigPairAttributes {
    /// An Azure config.
    #[serde(rename = "configs")]
    pub configs: Vec<crate::datadogV2::model::AzureUCConfig>,
    /// The ID of the Azure config pair.
    #[serde(rename = "id")]
    pub id: Option<i64>,
}

impl AzureUCConfigPairAttributes {
    pub fn new(
        configs: Vec<crate::datadogV2::model::AzureUCConfig>,
    ) -> AzureUCConfigPairAttributes {
        AzureUCConfigPairAttributes { configs, id: None }
    }

    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }
}
