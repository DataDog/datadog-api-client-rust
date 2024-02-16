// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The objects used to set an AWS tag filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSTagFilterCreateRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The namespace associated with the tag filter entry.
    #[serde(rename = "namespace")]
    pub namespace: Option<crate::datadogV1::model::AWSNamespace>,
    /// The tag filter string.
    #[serde(rename = "tag_filter_str")]
    pub tag_filter_str: Option<String>,
}

impl AWSTagFilterCreateRequest {
    pub fn new() -> AWSTagFilterCreateRequest {
        AWSTagFilterCreateRequest {
            account_id: None,
            namespace: None,
            tag_filter_str: None,
        }
    }

    pub fn account_id(&mut self, value: String) -> &mut Self {
        self.account_id = Some(value);
        self
    }

    pub fn namespace(&mut self, value: crate::datadogV1::model::AWSNamespace) -> &mut Self {
        self.namespace = Some(value);
        self
    }

    pub fn tag_filter_str(&mut self, value: String) -> &mut Self {
        self.tag_filter_str = Some(value);
        self
    }
}

impl Default for AWSTagFilterCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}
