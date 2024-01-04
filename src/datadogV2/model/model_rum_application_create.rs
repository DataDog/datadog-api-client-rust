// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application creation.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationCreate {
    /// RUM application creation attributes.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::RUMApplicationCreateAttributes>,
    /// RUM application creation type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::RUMApplicationCreateType,
}

impl RUMApplicationCreate {
    pub fn new(
        attributes: Box<crate::datadogV2::model::RUMApplicationCreateAttributes>,
        type_: crate::datadogV2::model::RUMApplicationCreateType,
    ) -> RUMApplicationCreate {
        RUMApplicationCreate { attributes, type_ }
    }
}
