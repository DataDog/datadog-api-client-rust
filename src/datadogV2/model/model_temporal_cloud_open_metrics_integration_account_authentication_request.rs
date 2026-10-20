// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication for creating the Temporal Cloud OpenMetrics integration account. Exactly one method is set.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TemporalCloudOpenMetricsIntegrationAccountAuthenticationRequest {
    TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest(Box<crate::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TemporalCloudOpenMetricsIntegrationAccountAuthenticationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest>>(value.clone()) {
			if !_v._unparsed {
                return Ok(TemporalCloudOpenMetricsIntegrationAccountAuthenticationRequest::TemporalCloudOpenMetricsIntegrationAccountBearerTokenAuthRequest(_v));
            }
        }

        return Ok(
            TemporalCloudOpenMetricsIntegrationAccountAuthenticationRequest::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
