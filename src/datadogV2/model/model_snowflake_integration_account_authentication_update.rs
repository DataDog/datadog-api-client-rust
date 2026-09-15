// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Authentication for updating the Snowflake integration account. Exactly one method is set. An update replaces the authentication object entirely, so it requires the same fields as creating an account.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SnowflakeIntegrationAccountAuthenticationUpdate {
    SnowflakeIntegrationAccountPrivateKeyAuthRequest(
        Box<crate::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthRequest>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SnowflakeIntegrationAccountAuthenticationUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthRequest>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SnowflakeIntegrationAccountAuthenticationUpdate::SnowflakeIntegrationAccountPrivateKeyAuthRequest(_v));
            }
        }

        return Ok(
            SnowflakeIntegrationAccountAuthenticationUpdate::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
