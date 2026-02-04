// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a flaky test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestAttributes {
    /// Unique identifier for the attempt to fix this flaky test. Use this ID in the Git commit message in order to trigger the attempt to fix workflow.
    ///
    /// When the workflow is triggered the test is automatically retried by the tracer a certain number of configurable times. When all retries pass, the test is automatically marked as fixed in Flaky Test Management.
    /// Test runs are tagged with @test.test_management.attempt_to_fix_passed and @test.test_management.is_attempt_to_fix when the attempt to fix workflow is triggered.
    #[serde(rename = "attempt_to_fix_id")]
    pub attempt_to_fix_id: Option<String>,
    /// The name of the test's code owners as inferred from the repository configuration.
    #[serde(rename = "codeowners")]
    pub codeowners: Option<Vec<String>>,
    /// List of environments where this test has been flaky.
    #[serde(rename = "envs")]
    pub envs: Option<Vec<String>>,
    /// The branch name where the test exhibited flakiness for the first time.
    #[serde(rename = "first_flaked_branch")]
    pub first_flaked_branch: Option<String>,
    /// The commit SHA where the test exhibited flakiness for the first time.
    #[serde(rename = "first_flaked_sha")]
    pub first_flaked_sha: Option<String>,
    /// Unix timestamp when the test exhibited flakiness for the first time.
    #[serde(rename = "first_flaked_ts")]
    pub first_flaked_ts: Option<i64>,
    /// The category of a flaky test.
    #[serde(
        rename = "flaky_category",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub flaky_category: Option<Option<String>>,
    /// The current state of the flaky test.
    #[serde(rename = "flaky_state")]
    pub flaky_state: Option<crate::datadogV2::model::FlakyTestAttributesFlakyState>,
    /// Chronological history of status changes for this flaky test, ordered from most recent to oldest.
    /// Includes state transitions like new -> quarantined -> fixed, along with the associated commit SHA when available.
    #[serde(rename = "history")]
    pub history: Option<Vec<crate::datadogV2::model::FlakyTestHistory>>,
    /// The branch name where the test exhibited flakiness for the last time.
    #[serde(rename = "last_flaked_branch")]
    pub last_flaked_branch: Option<String>,
    /// The commit SHA where the test exhibited flakiness for the last time.
    #[serde(rename = "last_flaked_sha")]
    pub last_flaked_sha: Option<String>,
    /// Unix timestamp when the test exhibited flakiness for the last time.
    #[serde(rename = "last_flaked_ts")]
    pub last_flaked_ts: Option<i64>,
    /// The name of the test module. The definition of module changes slightly per language:
    /// - In .NET, a test module groups every test that is run under the same unit test project.
    /// - In Swift, a test module groups every test that is run for a given bundle.
    /// - In JavaScript, the test modules map one-to-one to test sessions.
    /// - In Java, a test module groups every test that is run by the same Maven Surefire/Failsafe or Gradle Test task execution.
    /// - In Python, a test module groups every test that is run under the same `.py` file as part of a test suite, which is typically managed by a framework like `unittest` or `pytest`.
    /// - In Ruby, a test module groups every test that is run within the same test file, which is typically managed by a framework like `RSpec` or `Minitest`.
    #[serde(rename = "module", default, with = "::serde_with::rust::double_option")]
    pub module: Option<Option<String>>,
    /// The test name. A concise name for a test case. Defined in the test itself.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// CI pipeline related statistics for the flaky test. This information is only available if test runs are associated with CI pipeline events from CI Visibility.
    #[serde(rename = "pipeline_stats")]
    pub pipeline_stats: Option<crate::datadogV2::model::FlakyTestPipelineStats>,
    /// List of test service names where this test has been flaky.
    ///
    /// A test service is a group of tests associated with a project or repository. It contains all the individual tests for your code, optionally organized into test suites, which are like folders for your tests.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
    /// The name of the test suite. A group of tests exercising the same unit of code depending on your language and testing framework.
    #[serde(rename = "suite")]
    pub suite: Option<String>,
    /// Metadata about the latest failed test run of the flaky test.
    #[serde(rename = "test_run_metadata")]
    pub test_run_metadata: Option<crate::datadogV2::model::FlakyTestRunMetadata>,
    /// Test statistics for the flaky test.
    #[serde(rename = "test_stats")]
    pub test_stats: Option<crate::datadogV2::model::FlakyTestStats>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestAttributes {
    pub fn new() -> FlakyTestAttributes {
        FlakyTestAttributes {
            attempt_to_fix_id: None,
            codeowners: None,
            envs: None,
            first_flaked_branch: None,
            first_flaked_sha: None,
            first_flaked_ts: None,
            flaky_category: None,
            flaky_state: None,
            history: None,
            last_flaked_branch: None,
            last_flaked_sha: None,
            last_flaked_ts: None,
            module: None,
            name: None,
            pipeline_stats: None,
            services: None,
            suite: None,
            test_run_metadata: None,
            test_stats: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn attempt_to_fix_id(mut self, value: String) -> Self {
        self.attempt_to_fix_id = Some(value);
        self
    }

    pub fn codeowners(mut self, value: Vec<String>) -> Self {
        self.codeowners = Some(value);
        self
    }

    pub fn envs(mut self, value: Vec<String>) -> Self {
        self.envs = Some(value);
        self
    }

    pub fn first_flaked_branch(mut self, value: String) -> Self {
        self.first_flaked_branch = Some(value);
        self
    }

    pub fn first_flaked_sha(mut self, value: String) -> Self {
        self.first_flaked_sha = Some(value);
        self
    }

    pub fn first_flaked_ts(mut self, value: i64) -> Self {
        self.first_flaked_ts = Some(value);
        self
    }

    pub fn flaky_category(mut self, value: Option<String>) -> Self {
        self.flaky_category = Some(value);
        self
    }

    pub fn flaky_state(
        mut self,
        value: crate::datadogV2::model::FlakyTestAttributesFlakyState,
    ) -> Self {
        self.flaky_state = Some(value);
        self
    }

    pub fn history(mut self, value: Vec<crate::datadogV2::model::FlakyTestHistory>) -> Self {
        self.history = Some(value);
        self
    }

    pub fn last_flaked_branch(mut self, value: String) -> Self {
        self.last_flaked_branch = Some(value);
        self
    }

    pub fn last_flaked_sha(mut self, value: String) -> Self {
        self.last_flaked_sha = Some(value);
        self
    }

    pub fn last_flaked_ts(mut self, value: i64) -> Self {
        self.last_flaked_ts = Some(value);
        self
    }

    pub fn module(mut self, value: Option<String>) -> Self {
        self.module = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn pipeline_stats(
        mut self,
        value: crate::datadogV2::model::FlakyTestPipelineStats,
    ) -> Self {
        self.pipeline_stats = Some(value);
        self
    }

    pub fn services(mut self, value: Vec<String>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn suite(mut self, value: String) -> Self {
        self.suite = Some(value);
        self
    }

    pub fn test_run_metadata(
        mut self,
        value: crate::datadogV2::model::FlakyTestRunMetadata,
    ) -> Self {
        self.test_run_metadata = Some(value);
        self
    }

    pub fn test_stats(mut self, value: crate::datadogV2::model::FlakyTestStats) -> Self {
        self.test_stats = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for FlakyTestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestAttributesVisitor;
        impl<'a> Visitor<'a> for FlakyTestAttributesVisitor {
            type Value = FlakyTestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attempt_to_fix_id: Option<String> = None;
                let mut codeowners: Option<Vec<String>> = None;
                let mut envs: Option<Vec<String>> = None;
                let mut first_flaked_branch: Option<String> = None;
                let mut first_flaked_sha: Option<String> = None;
                let mut first_flaked_ts: Option<i64> = None;
                let mut flaky_category: Option<Option<String>> = None;
                let mut flaky_state: Option<
                    crate::datadogV2::model::FlakyTestAttributesFlakyState,
                > = None;
                let mut history: Option<Vec<crate::datadogV2::model::FlakyTestHistory>> = None;
                let mut last_flaked_branch: Option<String> = None;
                let mut last_flaked_sha: Option<String> = None;
                let mut last_flaked_ts: Option<i64> = None;
                let mut module: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut pipeline_stats: Option<crate::datadogV2::model::FlakyTestPipelineStats> =
                    None;
                let mut services: Option<Vec<String>> = None;
                let mut suite: Option<String> = None;
                let mut test_run_metadata: Option<crate::datadogV2::model::FlakyTestRunMetadata> =
                    None;
                let mut test_stats: Option<crate::datadogV2::model::FlakyTestStats> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attempt_to_fix_id" => {
                            if v.is_null() {
                                continue;
                            }
                            attempt_to_fix_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "codeowners" => {
                            if v.is_null() {
                                continue;
                            }
                            codeowners = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "envs" => {
                            if v.is_null() {
                                continue;
                            }
                            envs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_flaked_branch" => {
                            if v.is_null() {
                                continue;
                            }
                            first_flaked_branch =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_flaked_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            first_flaked_sha =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_flaked_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            first_flaked_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flaky_category" => {
                            flaky_category =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flaky_state" => {
                            if v.is_null() {
                                continue;
                            }
                            flaky_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _flaky_state) = flaky_state {
                                match _flaky_state {
                                    crate::datadogV2::model::FlakyTestAttributesFlakyState::UnparsedObject(_flaky_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "history" => {
                            if v.is_null() {
                                continue;
                            }
                            history = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_flaked_branch" => {
                            if v.is_null() {
                                continue;
                            }
                            last_flaked_branch =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_flaked_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            last_flaked_sha =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_flaked_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            last_flaked_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "module" => {
                            module = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pipeline_stats" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline_stats =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            if v.is_null() {
                                continue;
                            }
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suite" => {
                            if v.is_null() {
                                continue;
                            }
                            suite = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_run_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            test_run_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_stats" => {
                            if v.is_null() {
                                continue;
                            }
                            test_stats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FlakyTestAttributes {
                    attempt_to_fix_id,
                    codeowners,
                    envs,
                    first_flaked_branch,
                    first_flaked_sha,
                    first_flaked_ts,
                    flaky_category,
                    flaky_state,
                    history,
                    last_flaked_branch,
                    last_flaked_sha,
                    last_flaked_ts,
                    module,
                    name,
                    pipeline_stats,
                    services,
                    suite,
                    test_run_metadata,
                    test_stats,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestAttributesVisitor)
    }
}
