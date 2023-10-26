// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceUpdateAttributes {
    /// The custom URL for a custom region.
    #[serde(
        rename = "custom_url",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_url: Option<Option<String>>,
    /// The name for the Opsgenie service.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Opsgenie API key for your Opsgenie service.
    #[serde(rename = "opsgenie_api_key", skip_serializing_if = "Option::is_none")]
    pub opsgenie_api_key: Option<String>,
    /// The region for the Opsgenie service.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<crate::datadogV2::model::OpsgenieServiceRegionType>,
}

impl OpsgenieServiceUpdateAttributes {
    /// The Opsgenie service attributes for an update request.
    pub fn new() -> OpsgenieServiceUpdateAttributes {
        OpsgenieServiceUpdateAttributes {
            custom_url: None,
            name: None,
            opsgenie_api_key: None,
            region: None,
        }
    }
}
