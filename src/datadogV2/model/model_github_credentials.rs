// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Supported GitHub connection credentials.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GithubCredentials {
    GithubApp(Box<crate::datadogV2::model::GithubApp>),
    GithubPersonalAccessToken(Box<crate::datadogV2::model::GithubPersonalAccessToken>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for GithubCredentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::GithubApp>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(GithubCredentials::GithubApp(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GithubPersonalAccessToken>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(GithubCredentials::GithubPersonalAccessToken(_v));
            }
        }

        return Ok(GithubCredentials::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
