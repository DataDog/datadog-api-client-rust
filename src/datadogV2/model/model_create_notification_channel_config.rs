// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Defines the configuration for creating an On-Call notification channel
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CreateNotificationChannelConfig {
    CreatePhoneNotificationChannelConfig(
        Box<crate::datadogV2::model::CreatePhoneNotificationChannelConfig>,
    ),
    CreateEmailNotificationChannelConfig(
        Box<crate::datadogV2::model::CreateEmailNotificationChannelConfig>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CreateNotificationChannelConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CreatePhoneNotificationChannelConfig>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CreateNotificationChannelConfig::CreatePhoneNotificationChannelConfig(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CreateEmailNotificationChannelConfig>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    CreateNotificationChannelConfig::CreateEmailNotificationChannelConfig(_v),
                );
            }
        }

        return Ok(CreateNotificationChannelConfig::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
