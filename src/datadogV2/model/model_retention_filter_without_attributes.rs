// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterWithoutAttributes {
    /// The ID of the retention filter.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ApmRetentionFilterType,
}

impl RetentionFilterWithoutAttributes {
    /// The retention filter object .
    pub fn new(id: String, type_: crate::datadogV2::model::ApmRetentionFilterType) -> RetentionFilterWithoutAttributes {
        RetentionFilterWithoutAttributes { id: id, type_: type_ }
    }
}
