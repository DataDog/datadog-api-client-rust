// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Opsgenie service attributes for a create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceCreateAttributes {
    /// The custom URL for a custom region.
    #[serde(rename = "custom_url")]
    pub custom_url: Option<String>,
    /// The name for the Opsgenie service.
    #[serde(rename = "name")]
    pub name: String,
    /// The Opsgenie API key for your Opsgenie service.
    #[serde(rename = "opsgenie_api_key")]
    pub opsgenie_api_key: String,
    /// The region for the Opsgenie service.
    #[serde(rename = "region")]
    pub region: crate::datadogV2::model::OpsgenieServiceRegionType,
}

impl OpsgenieServiceCreateAttributes {
    pub fn new(
        name: String,
        opsgenie_api_key: String,
        region: crate::datadogV2::model::OpsgenieServiceRegionType,
    ) -> OpsgenieServiceCreateAttributes {
        OpsgenieServiceCreateAttributes {
            custom_url: None,
            name,
            opsgenie_api_key,
            region,
        }
    }

    pub fn custom_url(&mut self, value: String) -> &mut Self {
        self.custom_url = Some(value);
        self
    }
}
