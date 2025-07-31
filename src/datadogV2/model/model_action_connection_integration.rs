// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The definition of `ActionConnectionIntegration` object.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ActionConnectionIntegration {
    AWSIntegration(Box<crate::datadogV2::model::AWSIntegration>),
    AnthropicIntegration(Box<crate::datadogV2::model::AnthropicIntegration>),
    AsanaIntegration(Box<crate::datadogV2::model::AsanaIntegration>),
    AzureIntegration(Box<crate::datadogV2::model::AzureIntegration>),
    CircleCIIntegration(Box<crate::datadogV2::model::CircleCIIntegration>),
    ClickupIntegration(Box<crate::datadogV2::model::ClickupIntegration>),
    CloudflareIntegration(Box<crate::datadogV2::model::CloudflareIntegration>),
    ConfigCatIntegration(Box<crate::datadogV2::model::ConfigCatIntegration>),
    DatadogIntegration(Box<crate::datadogV2::model::DatadogIntegration>),
    FastlyIntegration(Box<crate::datadogV2::model::FastlyIntegration>),
    FreshserviceIntegration(Box<crate::datadogV2::model::FreshserviceIntegration>),
    GCPIntegration(Box<crate::datadogV2::model::GCPIntegration>),
    GeminiIntegration(Box<crate::datadogV2::model::GeminiIntegration>),
    GitlabIntegration(Box<crate::datadogV2::model::GitlabIntegration>),
    GreyNoiseIntegration(Box<crate::datadogV2::model::GreyNoiseIntegration>),
    HTTPIntegration(Box<crate::datadogV2::model::HTTPIntegration>),
    LaunchDarklyIntegration(Box<crate::datadogV2::model::LaunchDarklyIntegration>),
    NotionIntegration(Box<crate::datadogV2::model::NotionIntegration>),
    OktaIntegration(Box<crate::datadogV2::model::OktaIntegration>),
    OpenAIIntegration(Box<crate::datadogV2::model::OpenAIIntegration>),
    ServiceNowIntegration(Box<crate::datadogV2::model::ServiceNowIntegration>),
    SplitIntegration(Box<crate::datadogV2::model::SplitIntegration>),
    StatsigIntegration(Box<crate::datadogV2::model::StatsigIntegration>),
    VirusTotalIntegration(Box<crate::datadogV2::model::VirusTotalIntegration>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ActionConnectionIntegration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AWSIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::AWSIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::AnthropicIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::AnthropicIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AsanaIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::AsanaIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::AzureIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::AzureIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CircleCIIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::CircleCIIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ClickupIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::ClickupIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CloudflareIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::CloudflareIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ConfigCatIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::ConfigCatIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::DatadogIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::DatadogIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::FastlyIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::FastlyIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::FreshserviceIntegration>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::FreshserviceIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::GCPIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::GCPIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::GeminiIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::GeminiIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::GitlabIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::GitlabIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::GreyNoiseIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::GreyNoiseIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::HTTPIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::HTTPIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::LaunchDarklyIntegration>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::LaunchDarklyIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::NotionIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::NotionIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::OktaIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::OktaIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::OpenAIIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::OpenAIIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ServiceNowIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::ServiceNowIntegration(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::SplitIntegration>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::SplitIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::StatsigIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::StatsigIntegration(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::VirusTotalIntegration>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ActionConnectionIntegration::VirusTotalIntegration(_v));
            }
        }

        return Ok(ActionConnectionIntegration::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
