// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppResponsePage {
    /// The cursor to use to get the next results, if any. To make the next request, use the same parameters with the addition of `page[cursor]`.
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: String,
}

