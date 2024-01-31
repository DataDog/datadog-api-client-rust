// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AWS CUR config Patch data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfigPatchData {
    /// Attributes for AWS CUR config Patch Request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::AwsCURConfigPatchRequestAttributes,
    /// Type of AWS CUR config Patch Request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AwsCURConfigPatchRequestType,
}

impl AwsCURConfigPatchData {
    pub fn new(
        attributes: crate::datadogV2::model::AwsCURConfigPatchRequestAttributes,
        type_: crate::datadogV2::model::AwsCURConfigPatchRequestType,
    ) -> AwsCURConfigPatchData {
        AwsCURConfigPatchData { attributes, type_ }
    }
}
