// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Cloud Cost Activity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudCostActivity {
    /// Attributes for Cloud Cost activity.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::CloudCostActivityAttributes,
    /// Type of Cloud Cost Activity.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CloudCostActivityType,
}

impl CloudCostActivity {
    pub fn new(
        attributes: crate::datadogV2::model::CloudCostActivityAttributes,
        type_: crate::datadogV2::model::CloudCostActivityType,
    ) -> CloudCostActivity {
        CloudCostActivity { attributes, type_ }
    }
}
