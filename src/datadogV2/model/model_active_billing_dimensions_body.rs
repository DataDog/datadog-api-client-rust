// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Active billing dimensions data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveBillingDimensionsBody {
    /// List of active billing dimensions.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::ActiveBillingDimensionsAttributes>>,
    /// Unique ID of the response.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of active billing dimensions data.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ActiveBillingDimensionsType>,
}

impl ActiveBillingDimensionsBody {
    pub fn new() -> ActiveBillingDimensionsBody {
        ActiveBillingDimensionsBody {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for ActiveBillingDimensionsBody {
    fn default() -> Self {
        Self::new()
    }
}
