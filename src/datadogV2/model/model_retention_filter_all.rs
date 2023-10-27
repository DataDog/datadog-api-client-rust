// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterAll {
    /// The attributes of the retention filter.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::RetentionFilterAllAttributes>,
    /// The ID of the retention filter.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ApmRetentionFilterType,
}

impl RetentionFilterAll {
    /// The definition of the retention filter.
    pub fn new(
        attributes: crate::datadogV2::model::RetentionFilterAllAttributes,
        id: String,
        type_: crate::datadogV2::model::ApmRetentionFilterType,
    ) -> RetentionFilterAll {
        RetentionFilterAll {
            attributes: Box::new(attributes),
            id,
            type_,
        }
    }
}
