// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of `ActionConnectionIntegrationUpdate` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionConnectionIntegrationUpdate {
    AWSIntegrationUpdate(Box<crate::datadogV2::model::AWSIntegrationUpdate>),
    AnthropicIntegrationUpdate(Box<crate::datadogV2::model::AnthropicIntegrationUpdate>),
    AsanaIntegrationUpdate(Box<crate::datadogV2::model::AsanaIntegrationUpdate>),
    AzureIntegrationUpdate(Box<crate::datadogV2::model::AzureIntegrationUpdate>),
    CircleCIIntegrationUpdate(Box<crate::datadogV2::model::CircleCIIntegrationUpdate>),
    ClickupIntegrationUpdate(Box<crate::datadogV2::model::ClickupIntegrationUpdate>),
    CloudflareIntegrationUpdate(Box<crate::datadogV2::model::CloudflareIntegrationUpdate>),
    ConfigCatIntegrationUpdate(Box<crate::datadogV2::model::ConfigCatIntegrationUpdate>),
    DatadogIntegrationUpdate(Box<crate::datadogV2::model::DatadogIntegrationUpdate>),
    FastlyIntegrationUpdate(Box<crate::datadogV2::model::FastlyIntegrationUpdate>),
    FreshserviceIntegrationUpdate(Box<crate::datadogV2::model::FreshserviceIntegrationUpdate>),
    GCPIntegrationUpdate(Box<crate::datadogV2::model::GCPIntegrationUpdate>),
    GeminiIntegrationUpdate(Box<crate::datadogV2::model::GeminiIntegrationUpdate>),
    GitlabIntegrationUpdate(Box<crate::datadogV2::model::GitlabIntegrationUpdate>),
    GreyNoiseIntegrationUpdate(Box<crate::datadogV2::model::GreyNoiseIntegrationUpdate>),
    HTTPIntegrationUpdate(Box<crate::datadogV2::model::HTTPIntegrationUpdate>),
    LaunchDarklyIntegrationUpdate(Box<crate::datadogV2::model::LaunchDarklyIntegrationUpdate>),
    NotionIntegrationUpdate(Box<crate::datadogV2::model::NotionIntegrationUpdate>),
    OktaIntegrationUpdate(Box<crate::datadogV2::model::OktaIntegrationUpdate>),
    OpenAIIntegrationUpdate(Box<crate::datadogV2::model::OpenAIIntegrationUpdate>),
    ServiceNowIntegrationUpdate(Box<crate::datadogV2::model::ServiceNowIntegrationUpdate>),
    SplitIntegrationUpdate(Box<crate::datadogV2::model::SplitIntegrationUpdate>),
    StatsigIntegrationUpdate(Box<crate::datadogV2::model::StatsigIntegrationUpdate>),
    VirusTotalIntegrationUpdate(Box<crate::datadogV2::model::VirusTotalIntegrationUpdate>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionConnectionIntegrationUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AWSIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::AWSIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AnthropicIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::AnthropicIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AsanaIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::AsanaIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AzureIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::AzureIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CircleCIIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::CircleCIIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ClickupIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::ClickupIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CloudflareIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::CloudflareIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ConfigCatIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::ConfigCatIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::DatadogIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::DatadogIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::FastlyIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::FastlyIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::FreshserviceIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::FreshserviceIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::GCPIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::GCPIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GeminiIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::GeminiIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GitlabIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::GitlabIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GreyNoiseIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::GreyNoiseIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::HTTPIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::HTTPIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LaunchDarklyIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::LaunchDarklyIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::NotionIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::NotionIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::OktaIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::OktaIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::OpenAIIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::OpenAIIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ServiceNowIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::ServiceNowIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SplitIntegrationUpdate>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::SplitIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::StatsigIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::StatsigIntegrationUpdate(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::VirusTotalIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegrationUpdate::VirusTotalIntegrationUpdate(_v));
            }
        }

        return Ok(ActionConnectionIntegrationUpdate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
