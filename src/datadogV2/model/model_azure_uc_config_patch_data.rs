// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Azure config Patch data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureUCConfigPatchData {
    /// Attributes for Azure config Patch Request.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::AzureUCConfigPatchRequestAttributes>,
    /// Type of Azure config Patch Request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AzureUCConfigPatchRequestType,
}

impl AzureUCConfigPatchData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::AzureUCConfigPatchRequestAttributes>,
        type_: crate::datadogV2::model::AzureUCConfigPatchRequestType,
    ) -> AzureUCConfigPatchData {
        AzureUCConfigPatchData { attributes, type_ }
    }
}