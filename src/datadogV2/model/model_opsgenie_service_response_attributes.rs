// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The attributes from an Opsgenie service response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceResponseAttributes {
    /// The custom URL for a custom region.
    #[serde(
        rename = "custom_url",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub custom_url: Option<Option<String>>,
    /// The name for the Opsgenie service.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The region for the Opsgenie service.
    #[serde(rename = "region")]
    pub region: Option<crate::datadogV2::model::OpsgenieServiceRegionType>,
}

impl OpsgenieServiceResponseAttributes {
    pub fn new() -> OpsgenieServiceResponseAttributes {
        OpsgenieServiceResponseAttributes {
            custom_url: None,
            name: None,
            region: None,
        }
    }

    pub fn with_custom_url(&mut self, value: Option<String>) -> &mut Self {
        self.custom_url = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_region(
        &mut self,
        value: crate::datadogV2::model::OpsgenieServiceRegionType,
    ) -> &mut Self {
        self.region = Some(value);
        self
    }
}
impl Default for OpsgenieServiceResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
