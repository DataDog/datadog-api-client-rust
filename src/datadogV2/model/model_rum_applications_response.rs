// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM applications response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationsResponse {
    /// RUM applications array response.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::RUMApplicationList>>,
}

impl RUMApplicationsResponse {
    pub fn new() -> RUMApplicationsResponse {
        RUMApplicationsResponse { data: None }
    }

    pub fn with_data(
        &mut self,
        value: Vec<crate::datadogV2::model::RUMApplicationList>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for RUMApplicationsResponse {
    fn default() -> Self {
        Self::new()
    }
}
