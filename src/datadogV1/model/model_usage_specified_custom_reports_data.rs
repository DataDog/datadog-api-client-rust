// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing date and type for specified custom reports.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsData {
    /// The response containing attributes for specified custom reports.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV1::model::UsageSpecifiedCustomReportsAttributes>>,
    /// The date for specified custom reports.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of reports.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::UsageReportsType>,
}

impl UsageSpecifiedCustomReportsData {
    pub fn new() -> UsageSpecifiedCustomReportsData {
        UsageSpecifiedCustomReportsData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
