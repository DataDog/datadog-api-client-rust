// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The Opsgenie service attributes for an update request.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpsgenieServiceUpdateAttributes {
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
    /// The Opsgenie API key for your Opsgenie service.
    #[serde(rename = "opsgenie_api_key")]
    pub opsgenie_api_key: Option<String>,
    /// The region for the Opsgenie service.
    #[serde(rename = "region")]
    pub region: Option<crate::datadogV2::model::OpsgenieServiceRegionType>,
}

impl OpsgenieServiceUpdateAttributes {
    pub fn new() -> OpsgenieServiceUpdateAttributes {
        OpsgenieServiceUpdateAttributes {
            custom_url: None,
            name: None,
            opsgenie_api_key: None,
            region: None,
        }
    }
}