# CHANGELOG

## 0.18.0/2025-09-09

### Added
* Add Incident Notification Template Public Docs [#897](https://github.com/DataDog/datadog-api-client-rust/pull/897)
* Add Cross Org API to Open API specs [#896](https://github.com/DataDog/datadog-api-client-rust/pull/896)
* Update Get All Notification Rules API docs to include pagination, sorting, and filtering params [#894](https://github.com/DataDog/datadog-api-client-rust/pull/894)
* Add readonly ID of synthetics test steps [#891](https://github.com/DataDog/datadog-api-client-rust/pull/891)
* Create Cloud SIEM histsignals endpoints [#890](https://github.com/DataDog/datadog-api-client-rust/pull/890)
* Security Monitoring - Validation Endpoint for Suppressions [#887](https://github.com/DataDog/datadog-api-client-rust/pull/887)
* Security Monitoring - Related Suppressions for a Rule [#884](https://github.com/DataDog/datadog-api-client-rust/pull/884)
* Extend Widget time schema with support for hide_incomplete_cost_data [#874](https://github.com/DataDog/datadog-api-client-rust/pull/874)
* Add SDS rule `should_save_match` field [#872](https://github.com/DataDog/datadog-api-client-rust/pull/872)
* Add spec for Agentless GetAwsScanOptions [#869](https://github.com/DataDog/datadog-api-client-rust/pull/869)
* Add Cross Org API to Open API specs [#867](https://github.com/DataDog/datadog-api-client-rust/pull/867)
* Add DNAP Spark Pod Autosizing service to API client [#863](https://github.com/DataDog/datadog-api-client-rust/pull/863)
* Add version parameter to synthetic test trigger ci endpoint [#862](https://github.com/DataDog/datadog-api-client-rust/pull/862)
* Document Error Tracking public APIs [#859](https://github.com/DataDog/datadog-api-client-rust/pull/859)
* Add docs for 404 not found error in cost-onboarding-api [#854](https://github.com/DataDog/datadog-api-client-rust/pull/854)
* Adds async Scorecard outcomes batch update endpoint [#844](https://github.com/DataDog/datadog-api-client-rust/pull/844)

### Fixed
* Security Monitoring - Fix payload of Validation Endpoint for Suppressions [#892](https://github.com/DataDog/datadog-api-client-rust/pull/892)
* [CCA-938][CCC-883] Audit existing CCM endpoints in OpenAPI spec [#849](https://github.com/DataDog/datadog-api-client-rust/pull/849)
* Add enum Dataset type to Dataset API spec [#847](https://github.com/DataDog/datadog-api-client-rust/pull/847)

### Changed
* Update public cost permissions [#871](https://github.com/DataDog/datadog-api-client-rust/pull/871)
* Add Product Scales support to RUM v2 Applications API [#852](https://github.com/DataDog/datadog-api-client-rust/pull/852)

## 0.17.0/2025-08-12

### Added
* Add Flex_Logs_Compute_XL to API Spec [#833](https://github.com/DataDog/datadog-api-client-rust/pull/833)
* Support Host and IaC finding types in security notifications  [#832](https://github.com/DataDog/datadog-api-client-rust/pull/832)
* New keys for summary public endpoint for Event Management Correlation product [#828](https://github.com/DataDog/datadog-api-client-rust/pull/828)
* Add log autosubscription tag filters config to aws v2 API [#824](https://github.com/DataDog/datadog-api-client-rust/pull/824)
* Extended List Findings API to expose resource related Private IP Addresses to details [#817](https://github.com/DataDog/datadog-api-client-rust/pull/817)
* update metrics.yaml for ListMetricAssets and include Dashboard info [#812](https://github.com/DataDog/datadog-api-client-rust/pull/812)
* Support Cloud SIEM scheduled rules in API client [#809](https://github.com/DataDog/datadog-api-client-rust/pull/809)
* Uncomment edit dataset block, add dataset limitations into endpoint descriptions  [#807](https://github.com/DataDog/datadog-api-client-rust/pull/807)
* Add `text` field in synthetics search endpoint [#806](https://github.com/DataDog/datadog-api-client-rust/pull/806)
* Adding all action connection types to public API [#805](https://github.com/DataDog/datadog-api-client-rust/pull/805)
* Document case management attributes endpoints [#803](https://github.com/DataDog/datadog-api-client-rust/pull/803)
* add AP2 endpoint for On-Call Paging [#800](https://github.com/DataDog/datadog-api-client-rust/pull/800)
* Flag IP case action [#798](https://github.com/DataDog/datadog-api-client-rust/pull/798)
* Add DNS specs for Cloud Network Monitoring API [#796](https://github.com/DataDog/datadog-api-client-rust/pull/796)
* Adding Datadog Connection to Connection API [#790](https://github.com/DataDog/datadog-api-client-rust/pull/790)

### Fixed
* Split Dataset into separate request and response objects, mark unstable [#816](https://github.com/DataDog/datadog-api-client-rust/pull/816)
* Disables some tests to avoid fails as the service is disabled [#811](https://github.com/DataDog/datadog-api-client-rust/pull/811)
* OP make 'support_rules' field in parse_grok processor optional [#801](https://github.com/DataDog/datadog-api-client-rust/pull/801)

### Deprecated
* Deprecate signals triage v1 endpoints [#813](https://github.com/DataDog/datadog-api-client-rust/pull/813)

## 0.16.0/2025-07-14

### Added
* Add Datasets API to Open API Spec  [#777](https://github.com/DataDog/datadog-api-client-rust/pull/777)
* Add support for vulnerability management - GetSBOMsList new endpoint and update existing ones [#776](https://github.com/DataDog/datadog-api-client-rust/pull/776)
* Add spreadsheet to restriction_policy specs [#770](https://github.com/DataDog/datadog-api-client-rust/pull/770)
* Adds missing /api/v1/synthetics/tests/search spec [#764](https://github.com/DataDog/datadog-api-client-rust/pull/764)
* Add API spec for AWS Integrations IAM permissions [#758](https://github.com/DataDog/datadog-api-client-rust/pull/758)
* New keys added for multiple products [#756](https://github.com/DataDog/datadog-api-client-rust/pull/756)
* Add extractFromEmailBody step for synthetics browser test [#753](https://github.com/DataDog/datadog-api-client-rust/pull/753)
* Add support for `Array Processor` in `Logs Pipelines` [#751](https://github.com/DataDog/datadog-api-client-rust/pull/751)
* Update Incident API specs to include `is_test` in `POST /incidents` and incidents response [#744](https://github.com/DataDog/datadog-api-client-rust/pull/744)
* Add App Key Registration API  [#738](https://github.com/DataDog/datadog-api-client-rust/pull/738)

### Changed
* Update template variable schemas with type field in dashboards and shared dashboards endpoints for group by template variables [#752](https://github.com/DataDog/datadog-api-client-rust/pull/752)
* Update events intake specs for v2 Events post endpoint [#745](https://github.com/DataDog/datadog-api-client-rust/pull/745)

### Fixed
* Synthetics mobile test `message` field is now required [#750](https://github.com/DataDog/datadog-api-client-rust/pull/750)

### Removed
* Remove caseIndex from historical jobs api spec [#749](https://github.com/DataDog/datadog-api-client-rust/pull/749)

## 0.15.0/2025-06-30

### Fixed
* Synthetics mobile test `message` field is now required [#750](https://github.com/DataDog/datadog-api-client-rust/pull/750)
* Make dns port be string and number [#734](https://github.com/DataDog/datadog-api-client-rust/pull/734)
* Fix basic auth requirements [#731](https://github.com/DataDog/datadog-api-client-rust/pull/731)
* Add support for the api_security detection rule type [#726](https://github.com/DataDog/datadog-api-client-rust/pull/726)

### Security
* Remove caseIndex from historical jobs api spec [#749](https://github.com/DataDog/datadog-api-client-rust/pull/749)

### Changed
* Update events intake specs for v2 Events post endpoint [#745](https://github.com/DataDog/datadog-api-client-rust/pull/745)
* Update events intake specs for v2 Events post endpoint [#707](https://github.com/DataDog/datadog-api-client-rust/pull/707)
* Add billing read permission [#702](https://github.com/DataDog/datadog-api-client-rust/pull/702)

### Added
* Update Incident API specs to include `is_test` in `POST /incidents` and incidents response [#744](https://github.com/DataDog/datadog-api-client-rust/pull/744)
* Add App Key Registration API  [#738](https://github.com/DataDog/datadog-api-client-rust/pull/738)
* Microsoft Sentinel Public API support [#729](https://github.com/DataDog/datadog-api-client-rust/pull/729)
* Add the AP2 datacenter. [#727](https://github.com/DataDog/datadog-api-client-rust/pull/727)
* Add custom fields to Rule update/validate API public documentation. [#718](https://github.com/DataDog/datadog-api-client-rust/pull/718)
* Add hash field to actions in CWS agent rules [#715](https://github.com/DataDog/datadog-api-client-rust/pull/715)

### Deprecated
* Deprecate SLO metadata fields in api spec [#700](https://github.com/DataDog/datadog-api-client-rust/pull/700)

## 0.14.0/2025-06-24

### Fixed
* Fix basic auth requirements [#731](https://github.com/DataDog/datadog-api-client-rust/pull/731)
* Add support for the api_security detection rule type [#726](https://github.com/DataDog/datadog-api-client-rust/pull/726)

### Added
* Microsoft Sentinel Public API support [#729](https://github.com/DataDog/datadog-api-client-rust/pull/729)
* Add the AP2 datacenter. [#727](https://github.com/DataDog/datadog-api-client-rust/pull/727)
* Add custom fields to Rule update/validate API public documentation. [#718](https://github.com/DataDog/datadog-api-client-rust/pull/718)
* Add hash field to actions in CWS agent rules [#715](https://github.com/DataDog/datadog-api-client-rust/pull/715)
* Add `form` field for `multipart/form-data` HTTP API tests [#698](https://github.com/DataDog/datadog-api-client-rust/pull/698)
* SDCD-1142: adding `custom_tags` optional attribute to DORA API spec [#697](https://github.com/DataDog/datadog-api-client-rust/pull/697)
* Add sampling fields to SDS spec [#693](https://github.com/DataDog/datadog-api-client-rust/pull/693)
* Add new endpoint to upsert/list/delete custom kinds [#692](https://github.com/DataDog/datadog-api-client-rust/pull/692)
* Add spec for team on-call endpoint [#690](https://github.com/DataDog/datadog-api-client-rust/pull/690)

### Changed
* Update events intake specs for v2 Events post endpoint [#707](https://github.com/DataDog/datadog-api-client-rust/pull/707)
* Add billing read permission [#702](https://github.com/DataDog/datadog-api-client-rust/pull/702)

## 0.13.0/2025-06-23

### Fixed
* Fix basic auth requirements [#731](https://github.com/DataDog/datadog-api-client-rust/pull/731)
* Add support for the api_security detection rule type [#726](https://github.com/DataDog/datadog-api-client-rust/pull/726)

### Added
* Microsoft Sentinel Public API support [#729](https://github.com/DataDog/datadog-api-client-rust/pull/729)
* Add custom fields to Rule update/validate API public documentation. [#718](https://github.com/DataDog/datadog-api-client-rust/pull/718)
* Add hash field to actions in CWS agent rules [#715](https://github.com/DataDog/datadog-api-client-rust/pull/715)
* Add `form` field for `multipart/form-data` HTTP API tests [#698](https://github.com/DataDog/datadog-api-client-rust/pull/698)
* SDCD-1142: adding `custom_tags` optional attribute to DORA API spec [#697](https://github.com/DataDog/datadog-api-client-rust/pull/697)
* Add sampling fields to SDS spec [#693](https://github.com/DataDog/datadog-api-client-rust/pull/693)
* Add new endpoint to upsert/list/delete custom kinds [#692](https://github.com/DataDog/datadog-api-client-rust/pull/692)
* Add spec for team on-call endpoint [#690](https://github.com/DataDog/datadog-api-client-rust/pull/690)

### Changed
* Update events intake specs for v2 Events post endpoint [#707](https://github.com/DataDog/datadog-api-client-rust/pull/707)
* Add billing read permission [#702](https://github.com/DataDog/datadog-api-client-rust/pull/702)

## 0.12.0/2025-06-16

### Changed
* Add billing read permission [#702](https://github.com/DataDog/datadog-api-client-rust/pull/702)
* Update DORA endpoints [#682](https://github.com/DataDog/datadog-api-client-rust/pull/682)

### Added
* Add `form` field for `multipart/form-data` HTTP API tests [#698](https://github.com/DataDog/datadog-api-client-rust/pull/698)
* Add new endpoint to upsert/list/delete custom kinds [#692](https://github.com/DataDog/datadog-api-client-rust/pull/692)
* Add spec for team on-call endpoint [#690](https://github.com/DataDog/datadog-api-client-rust/pull/690)
* Add support for Datadog Events as a data source for rules [#670](https://github.com/DataDog/datadog-api-client-rust/pull/670)
* Add public APIs to search DORA events [#668](https://github.com/DataDog/datadog-api-client-rust/pull/668)
* Add support for all subtypes in multistep steps [#665](https://github.com/DataDog/datadog-api-client-rust/pull/665)
* Added new optional field definition to include more detail in findings for '/api/v2/posture_management/findings'  [#663](https://github.com/DataDog/datadog-api-client-rust/pull/663)
* Exposing set action on Terraform V2 [#662](https://github.com/DataDog/datadog-api-client-rust/pull/662)
* Add monitor draft status field [#661](https://github.com/DataDog/datadog-api-client-rust/pull/661)

### Fixed
* add `include` parameter to On-Call team rules test [#675](https://github.com/DataDog/datadog-api-client-rust/pull/675)
* fix On-Call spec [#667](https://github.com/DataDog/datadog-api-client-rust/pull/667)

## 0.11.0/2025-05-28

### Fixed
* add `include` parameter to On-Call team rules test [#675](https://github.com/DataDog/datadog-api-client-rust/pull/675)
* fix On-Call spec [#667](https://github.com/DataDog/datadog-api-client-rust/pull/667)
* Fix incorrect pattern for url [#651](https://github.com/DataDog/datadog-api-client-rust/pull/651)
* Make metadata optional for GCS destination [#641](https://github.com/DataDog/datadog-api-client-rust/pull/641)
* Remove isReadOnly default when creating dashboards [#640](https://github.com/DataDog/datadog-api-client-rust/pull/640)
* Make assertion target be int or string [#637](https://github.com/DataDog/datadog-api-client-rust/pull/637)

### Added
* Add support for Datadog Events as a data source for rules [#670](https://github.com/DataDog/datadog-api-client-rust/pull/670)
* Add public APIs to search DORA events [#668](https://github.com/DataDog/datadog-api-client-rust/pull/668)
* Adding endpoints to manage Resource Evaluation Filters [#658](https://github.com/DataDog/datadog-api-client-rust/pull/658)
* Add Sev0 as a supported incident severity [#654](https://github.com/DataDog/datadog-api-client-rust/pull/654)
* Share timerestriction object [#650](https://github.com/DataDog/datadog-api-client-rust/pull/650)
* add On-Call Paging spec [#644](https://github.com/DataDog/datadog-api-client-rust/pull/644)
* Add pagination method for NDM ListDevices. [#638](https://github.com/DataDog/datadog-api-client-rust/pull/638)

## 0.10.1 / 2025-04-14

### Fixed
* Fix get_operation_host by @therve in https://github.com/DataDog/datadog-api-client-rust/pull/558
* Change `type` to enum to discriminate included items in the response of `ListCatalogEntity` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/568
* Deprecate options from logs aggregate API public spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/576
* change a category in enum for datadog_appsec_waf_custom_rule by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/570
### Added
* Add datasource to job definition for security monitoring  by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/563
* Include new rum types in Usage_metering Yaml by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/566
* Adding new UT apm_error_events keys in summary endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/560
* Add more triggers for workflow automation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/571
* Add specs for Cloud Network Monitoring API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/580
* Add more Security Monitoring Data Source enum values by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/575
* Add componentOf field to Service, Queue, and Datastore V3 Software Catalog definitions by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/578
* Add 'mute_buttons' argument to slack channel definition by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/581
* Add Observability Pipelines API  by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/586
* add rum slo bugfix by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/589
* Add trace_rate support to APM retention filter APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/593
* Update NDM GetInterfaces documentation to add ip_addresses attribute by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/592
* Add assertRequests browser step type by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/597
* Add user behavior case actions in API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/596
### Changed
* Remove OpenAPI enum enforcement of Service Definition v2dot2 type field from service definition endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/564
* Add on-call schedules endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/584

## New Contributors
* @therve made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/558

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.9.1...v0.10.1

## 0.9.1 / 2025-03-11

### Changed
* Remove meta from RUM retention filters APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/556


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.9.0...v0.9.1

## 0.9.0 / 2025-03-11

### Fixed
* Remove `javascript` browser variable type by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/506
* Additional rules to inject openapi type by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/525
* Fix `ListCatalogEntity` pagination endpoint to use correct offset value by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/543
### Added
* add new related_assets filter query parameter to the get a list of metrics V2 API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/474
* Add actions and groupSignalsBy field in detection rules API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/508
* Add Workflows CRUD Public API Endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/499
* Add endpoint to retrieve Security Monitoring rule version history by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/480
* Adds override_existing_configurations and include_actively_queried_configurations to bulk tag config endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/516
* Add `number_format` to each formula in widget by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/518
* Add `trend` support for `cell_display_mode` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/515
* Add support for span id remapper in logs pipelines processors by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/526
* Add evaluation_window and keep_alive for Security monitoring rule by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/527
* Add `extractedValuesFromScript` to multistep API tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/531
* Update timezone for cumulative window by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/530
* Document Agentless AWS scan options routes by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/538
* Create types for app builder queries explicitly, remove experimental flag by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/539
* Document Agentless AWS on demand routes by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/544
* Add quality_issues to monitor schema on monitor search API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/546
* Introduce public v2 endpoints for Application Security by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/532
* Add delete log index to public API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/545
* Add v2 endpoints for RUM retention filters. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/541
* Added storage class information to the S3 archive destination by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/550
### Changed
* Revert GetSBOM to `x-unstable` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/510
* Update documentation with account filtering info for aws_cur_config endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/521
* Update sharing APIs to match server by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/529
* Update Vulnerabilities endpoints documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/533
### Deprecated
* Deprecate API management endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/520


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.8.0...v0.9.0

## 0.8.0 / 2025-02-05

### Fixed
* Modify owner properties to be a string by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/500
### Added
* Add UT breakdown for fargate_container_profiler billing dimension by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/432
* Add synthetics browser step public_id field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/313
* Add support for vulnerability management  by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/433
* add start_date to suppression APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/398
* Add CSM Coverage Analysis API specs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/437
* Ephemeral Infra_host new keys in summary endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/446
* Update app builder API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/441
* Add meta and source fields to JSONAPIErrorItem by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/440
* Add CSM Agentless Read Endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/438
* Update rum doc to include new usage types by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/449
* add cost monitor type to API Spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/461
* Add Action Connection API for Workflow Automation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/447
* Add `type` in Data Deletion API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/465
* Add a cost monitor example by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/469
* Add `provider_name` attribute to pipelines API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/451
* Add support for vulnerability management - GetSBOM new endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/472
* Remove preview status for GetBillingDimensionMapping endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/479
* Add encryption field to logs archive destination by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/477
* Add tags and description to logs pipelines by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/482
* Publish security notification rules API endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/475
* Publish app builder API documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/453
* update public document with configuration event type by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/490
* Add support for Entity kind API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/495
* Rename `embeddedQueries` attribute to `queries` in app builder api by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/496
### Changed
* Fix specification for Azure metric filtering by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/435
* Change allow_self_lockout from string to bool by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/448
* remove flag Beta for cost-by-tag endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/487
* Added Support for Workflow Webhooks Public API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/491
* Vulnerabilities endpoints GA - Remove `x-unstable` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/497


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.7.0...v0.8.0

## 0.7.0 / 2024-12-17

### Fixed
* Fix example primitive oneOf variants getting option wrapped by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/427
### Added
* Adds accepted reasons for archiving signal by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/292
* Add usage type breakdown for error tracking billing dimension by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/368
* Add Historical Job endpoints to Datadog API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/367
* Add new keys for CWS Fargate Task in summary usage and usage attribution endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/371
* Add missing measures for SLOs data source by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/372
* Create AWS Integrations v2 API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/129
* Add step_functions as valid enum for v1 AWS tag filter spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/378
* Fix authz scope descriptions by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/388
* Updated OpenAPI logs_pattern_query to support Patterns for any attribute by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/387
* Add API specification for events intake v2 by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/334
* Data Deletion Endpoints Documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/374
* Add `exitIfSucceed` to multistep API tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/413
* Security Monitoring Rule - Add the updatedAt field in the SecurityMonitoringStandardRuleResponse by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/390
* add docs for pagination in /api/v2/metrics endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/416
* Add daily as a valid enum for SLOReportInterval by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/423
* Add new product Code Security host for summary endpoint and UA endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/392
* Add CSM Agents Read Endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/421
* Add app builder API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/415
### Changed
* Fix spelling error for bindings by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/369
* Revert the earlier spelling change by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/373
* Remove mobile device ids and make all device ids simple string by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/384
* Remove support for `namespace_filters.include/exclude_all` in v2 AWS Integrations API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/386
* Add running pipelines on custom pipelines API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/381
### Removed
* Remove unnecessary field in list stream column config by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/424
### Deprecated
* Remove `/api/v2/cost/enabled` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/410

## New Contributors
* @bthuilot made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/419

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.6.0...v0.7.0

## 0.6.0 / 2024-11-07

### Fixed
* Fix Toplist widget's stacked display style - remove legend as required field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/314
* Remove user fields that are unsupported by the Incidents API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/324
* Fix Synthetics batch status by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/362
### Added
* Add MSTeams integration metadata info by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/326
* Add `code_analysis_sa_committers_hwm` and `code_analysis_sca_committers_hwm` to UsageMetering by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/327
* Update GCP API Spec to support `is_resource_change_collection_enabled` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/330
* Add vulnerability type to Findings API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/329
* Update Documentation for Data Stream Monitoring by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/336
* Add LLM Observability to ListStreamSource by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/335
* Add synthetics stepDetail.allowFailure and stepDetail.failure by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/344
* Integrate incident types into Incidents API documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/328
* Add `use_recommended_keywords` attribute to sensitive data scanner rule spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/354
* Add domain allowlist endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/351
* Add v2 endpoints for RUM custom metrics. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/350
* Documentation for beta /v2/usage/billing_dimension_mapping by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/282
* Add `alwaysExecute` and `exitIfSucceed` to Synthetics steps by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/363
* Add metric_namespace_configs to GCP v2 API by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/365
### Changed
* Edit Naming for v2 Microsoft Teams Integration Endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/325
* Change the mobile device ids from enum to string by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/323
* Mark Cost Attribution end_month parameter as not required by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/310
* Allow for any type for additionalProperties in HTTPLogItem by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/343
* Make some amendments to the new mobiles schema by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/342
* Make value be oneOf number or string by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/355
* Add examples for resources for Cloudflare by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/349
### Removed
* Remove deprecated estimated usage types for usage attribution by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/347
### Deprecated
* Deprecate two sds metadata fields by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/341
* Delete `api/v2/cost/aws_related_accounts` from spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/356
* Deprecate `api/v2/cost/enabled` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/358


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.5.0...v0.6.0

## 0.5.0 / 2024-10-02

### Fixed
* change schema used in FastlyServicesResponse by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/304
### Added
* Add new synthetics HTTP javascript assertion by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/229
* Dashboards - Toplist widget style - Add palette by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/278
* Allow Table Widget requests to specify text replace formatting in dashboards by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/279
* Add documentation for Data Jobs Monitoring summary keys by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/284
* Update estimate docs with realtime changes by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/308
* Ensure clients can handle empty oneOf objects by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/305
* Add referenceTables field to security monitoring endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/301
* Add UA documentation for new DJM usage_type by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/302
* Add v2 endpoints for MS Teams Integration by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/311
* Add documention for OCI Integration by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/317
* Add schema for mobile test by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/293
* Add Synthetics endpoint to fetch uptimes in API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/272
### Changed
* Split the synthetics request port field into a oneOf by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/289
* Remove unused field `color` in `TeamUpdateAttributes` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/285
* Powerpack add support for prefix and available values by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/294
* Support DD_SITE env var and document server variable usage by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/306
* Update v2 metrics list endpoint filter by metric type to use metric type category by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/309


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.4.0...v0.5.0

## 0.4.0 / 2024-09-04

### Fixed
* Add `is_totp` and `is_fido` to Synthetic global variables by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/197
### Added
* Add `api_key` and `name` to `CloudflareAccountResponseAttributes`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/241
* Add `api_key` and `name` to `FastlyAccountUpdateRequestAttributes`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/242
* Add `opsgenie_api_key` to `OpsgenieServiceResponseAttributes`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/243
* Add `category` and `remote_config_read_enabled` to `APIKeyCreateAttributes`, and add `LeakedKey`. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/244
* Allow 4 group-bys for pattern viz by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/250
* add url attribute to metrics assets v2 api by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/252
* Add editable field to suppression rule by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/161
* Handle nested oneOf models by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/259
* Add `num_flex_logs_retention_days` field to logs_indexes api spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/253
* Software catalog openapi spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/260
### Changed
* include additionalProperties by default and check when false by @amaskara-dd in https://github.com/DataDog/datadog-api-client-rust/pull/249
* allow variables in port by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/236
* Fix VFTs and extracted local variables enum types by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/218
* Changed Widget time schema to add support for new fixed_span and live_span object by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/239
### Deprecated
* mark groupby_simple_monitor as deprecated by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/269


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.3.0...v0.4.0

## 0.3.0 / 2024-08-12

### Added
* Documentation for new device tags endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/213
* Update documentation for Flex Logs Starter by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/227
### Changed
* add mfa_enabled field and change created_at type to datetime by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/228


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.2.0...v0.3.0

## 0.2.0 / 2024-08-01

### Fixed
* fix monitor enum by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/183
* dashboards add support for time-slice SLOs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/184
* Make modified by field nullable for get all API keys by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/189
* Add `409 Conflict` to `CreateGlobalVariable` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/201
### Added
* add cross org uuids to timeseries query by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/149
* Add network performance monitor type to API spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/162
* Document `force_delete_dependencies` for synthetics test deletion by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/153
* Support metric filtering in integration azure GET, PUT APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/167
* add enableProfiling and enableSecurityTesting options by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/176
* Add convert rule JSON to terraform to Datadog API Spec. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/151
* add changes for datadog partner program to estimated cost and billable usage APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/155
* Add type as a required field for the different basic auth types by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/169
* Adding Network Device Monitoring API Documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/164
* Security Monitoring - Support anomaly threshold detection method by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/181
* update hourly usage API docs for partner program by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/190
* Add resource_type query param to authn mapping spec by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/128
* Add rum stream to API definition by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/196
* Support `incident_analytics` enum in dashboard widget `FormulaAndFunctionEventsDataSource` data sources by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/208
* update usage summary API docs for partner program by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/205
* update historical_cost and projected_cost for partner program by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/194
* Add custom cost endpoints to public API documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/209
* Update documentation for Cloud SIEM Analyzed Logs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/211
* Update documentation for App Sec SCA by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/199
* Add trigger API documentation for workflow automation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/204
* Add PUT endpoint to scorecards APIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/219
### Changed
* Rename examples with OperationIDs by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/192
### Deprecated
* Deprecate `ListAWSRelatedAccounts` by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/200

## New Contributors
* @amaskara-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/188

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.1.3...v0.2.0

## 0.1.3 / 2024-07-01

### Fixed
* Security Monitoring - Define specific payload for rule validation/testing by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/126
* Remove the maximum limitation for the synthetics renotify_interval monitor option by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/130
* Add bodyHash as a synthetics assertion type. by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/131
* Add missing attributes envelope in ListAPIs response by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/140
### Added
* Allow the usage of the filters field when creating an agent rule by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/106
* Add tileDef sort attribute by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/108
* Add Security Monitoring rule test endpoint by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/122
* Add originalFileName field to the SyntheticsTestRequestBodyFile definition by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/127
* Add support for API management ListAPIs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/134
* Add elementsOperator to json path assertion for synthetic HTTP tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/146
* Add /api/v2/org_configs specs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/145
* Support rustls with feature flag via reqwest by @skeggse in https://github.com/DataDog/datadog-api-client-rust/pull/158
* Update docs for RU Rollout New and Deprecated Keys planned for Oct 1st by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/160
* Add option for wait step in multistep api tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/159
### Changed
* Monitor priority can have custom ranges and be null by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/154

## New Contributors
* @jack-edmonds-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/150
* @skeggse made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/158

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.0.3...v0.1.3

## 0.0.3 / 2024-05-22

### Fixed
* Fix incorrect propagation of nullable rendering within additionalProperties by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/96
### Added
* Add support variablesFromScript in Synthetics API test by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/83
* Add JSONSchema assertion support to API and multistep tests by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/52
* add 1 day logs to usage api docs by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/90
* Update UserTeamIncluded to include teams by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/95
* Security Monitoring - Make Default Tags available in the response by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/105
* Add flex logs storage tier by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/107
### Changed
* Rename the Cloud Workload Security tag to CSM Threats by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/94
* Update license after code generation by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/110


**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/compare/0.0.2...v0.0.3

## 0.0.2 / 2024-04-16

### Fixed
* Build form data request body manually by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/63
* fix case search documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/80
### Added
* Add UA documentation for online_archive and incident_management by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/62
* Mark `unit` as nullable by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/65
* Add query_interval_seconds to time-slice SLO condition parameters by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/68
* Support providing files for the file upload feature when creating a Synthetic API test by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/67
* Documentation upgrades by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/66
* Adding SLO Reporting API Documentation by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/50
* Security Monitoring Suppression - Add data_exclusion_query field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/73
* aws api adding extended and deprecating old resource collection field by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/71
### Changed
* Add Team relationship to AuthNMappings by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/60
* Refactor imports by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/69
* Handle datetime format by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/41
* Add UUID support by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/47
* Replace duplicate error enums with a single variant per schema by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/77

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/commits/v0.0.2

## 0.0.1 / 2024-03-28

### Fixed
* Always serialize required nullable fields by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/16
* Fix edge case naming by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/18
* Update license check script and actions CI by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/32
* Custom de/serializing handling for enums by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/31
* Optimize example generation by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/40
### Added
* Add all APIs without oneOfs by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/5
* Add support for `unstableOperations` by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/12
* Add `unparsedObject` support by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/33
* Remove deprecated warnings by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/34
* Add proxy support by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/39
* Add validation endpoint for Security Monitoring Rules by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/58
* Add retry by @HantingZhang2 in https://github.com/DataDog/datadog-api-client-rust/pull/42
### Changed
* Introduce API instance pattern by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/3
* Add cassette record/replay/passthrough modes in bdd runner by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/4
* Implement OneOfs and remove default on enums by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/6
* Bugfixes based on BDD testing by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/7
* Add builder-like pattern to API instances and models by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/9
* End user usability refactors by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/11
* Configurable server variables by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/15
* Remove Option wrapper in successful responses by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/20
* Remove `x-generate-alias-as-model` support by @skarimo in https://github.com/DataDog/datadog-api-client-rust/pull/25
* Example generation by @nkzou in https://github.com/DataDog/datadog-api-client-rust/pull/27
### Deprecated
* Remove deprecated /api/v1/usage/attribution by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/45
* Deprecate legacy hourly usage metering endpoints by @api-clients-generation-pipeline in https://github.com/DataDog/datadog-api-client-rust/pull/53

## New Contributors
* @nkzou made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/1
* @skarimo made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/13
* @HantingZhang2 made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/17
* @antonio-ramadas-dd made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/37
* @api-clients-generation-pipeline made their first contribution in https://github.com/DataDog/datadog-api-client-rust/pull/38

**Full Changelog**: https://github.com/DataDog/datadog-api-client-rust/commits/2b9c33b
