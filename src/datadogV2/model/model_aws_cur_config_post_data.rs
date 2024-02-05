// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AWS CUR config Post data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfigPostData {
    /// Attributes for AWS CUR config Post Request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::AwsCURConfigPostRequestAttributes,
    /// Type of AWS CUR config Post Request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AwsCURConfigPostRequestType,
}

impl AwsCURConfigPostData {
    pub fn new(
        attributes: crate::datadogV2::model::AwsCURConfigPostRequestAttributes,
        type_: crate::datadogV2::model::AwsCURConfigPostRequestType,
    ) -> AwsCURConfigPostData {
        AwsCURConfigPostData { attributes, type_ }
    }
}
