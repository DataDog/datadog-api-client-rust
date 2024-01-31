// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AWS CUR config.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfig {
    /// Attributes for An AWS CUR config.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::AwsCURConfigAttributes,
    /// The ID of the AWS CUR config.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Type of AWS CUR config.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AwsCURConfigType,
}

impl AwsCURConfig {
    pub fn new(
        attributes: crate::datadogV2::model::AwsCURConfigAttributes,
        type_: crate::datadogV2::model::AwsCURConfigType,
    ) -> AwsCURConfig {
        AwsCURConfig {
            attributes,
            id: None,
            type_,
        }
    }

    pub fn with_id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }
}
