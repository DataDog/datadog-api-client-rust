// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The RUM data collection settings for the Synthetic browser test.
/// **Note:** There are 3 ways to format RUM settings:
///
/// `{ isEnabled: false }`
/// RUM data is not collected.
///
/// `{ isEnabled: true }`
/// RUM data is collected from the Synthetic test's default application.
///
/// `{ isEnabled: true, applicationId: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", clientTokenId: 12345 }`
/// RUM data is collected using the specified application.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestRumSettings {
    /// RUM application ID used to collect RUM data for the browser test.
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    /// RUM application API key ID used to collect RUM data for the browser test.
    #[serde(rename = "clientTokenId")]
    pub client_token_id: Option<i64>,
    /// Determines whether RUM data is collected during test runs.
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}

impl SyntheticsBrowserTestRumSettings {
    pub fn new(is_enabled: bool) -> SyntheticsBrowserTestRumSettings {
        SyntheticsBrowserTestRumSettings {
            application_id: None,
            client_token_id: None,
            is_enabled,
        }
    }

    pub fn application_id(&mut self, value: String) -> &mut Self {
        self.application_id = Some(value);
        self
    }

    pub fn client_token_id(&mut self, value: i64) -> &mut Self {
        self.client_token_id = Some(value);
        self
    }
}
