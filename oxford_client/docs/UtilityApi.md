# \UtilityApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domains_source_domains_language_target_domains_language_get**](UtilityApi.md#domains_source_domains_language_target_domains_language_get) | **Get** /domains/{source_domains_language}/{target_domains_language} | Lists available domains in a bilingual dataset
[**domains_source_language_get**](UtilityApi.md#domains_source_language_get) | **Get** /domains/{source_language} | Lists available domains in a monolingual dataset
[**filters_endpoint_get**](UtilityApi.md#filters_endpoint_get) | **Get** /filters/{endpoint} | Lists available filters for specific endpoint
[**filters_get**](UtilityApi.md#filters_get) | **Get** /filters | Lists available filters
[**grammatical_features_source_language_get**](UtilityApi.md#grammatical_features_source_language_get) | **Get** /grammaticalFeatures/{source_language} | Lists available grammatical features in a dataset
[**languages_get**](UtilityApi.md#languages_get) | **Get** /languages | Lists available dictionaries
[**lexicalcategories_language_get**](UtilityApi.md#lexicalcategories_language_get) | **Get** /lexicalcategories/{language} | Lists available lexical categories in a dataset
[**regions_source_language_get**](UtilityApi.md#regions_source_language_get) | **Get** /regions/{source_language} | Lists available regions in a monolingual dataset
[**registers_source_language_get**](UtilityApi.md#registers_source_language_get) | **Get** /registers/{source_language} | Lists available registers in a  monolingual dataset
[**registers_source_register_language_target_register_language_get**](UtilityApi.md#registers_source_register_language_target_register_language_get) | **Get** /registers/{source_register_language}/{target_register_language} | Lists available registers in a bilingual dataset


# **domains_source_domains_language_target_domains_language_get**
> ::models::UtilityLabels domains_source_domains_language_target_domains_language_get(source_domains_language, target_domains_language, app_id, app_key)
Lists available domains in a bilingual dataset

Returns a list of the available [domains](documentation/glossary?term=domain) for a given bilingual language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_domains_language** | **String**| IANA language code | 
  **target_domains_language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::UtilityLabels**](UtilityLabels.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **domains_source_language_get**
> ::models::UtilityLabels domains_source_language_get(source_language, app_id, app_key)
Lists available domains in a monolingual dataset

Returns a list of the available [domains](documentation/glossary?term=domain) for a given monolingual language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::UtilityLabels**](UtilityLabels.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **filters_endpoint_get**
> ::models::Filters filters_endpoint_get(endpoint, app_id, app_key)
Lists available filters for specific endpoint

Returns a list of all the valid filters for a given endpoint to construct API calls. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **endpoint** | **String**| Name of the endpoint. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Filters**](Filters.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **filters_get**
> ::models::Filters filters_get(app_id, app_key)
Lists available filters

Returns a list of all the valid filters to construct API calls. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Filters**](Filters.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **grammatical_features_source_language_get**
> ::models::UtilityLabels grammatical_features_source_language_get(source_language, app_id, app_key)
Lists available grammatical features in a dataset

Returns a list of the available [grammatical features](documentation/glossary?term=grammaticalfeatures) for a given language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_language** | **String**| IANA language code. If provided output will be filtered by sourceLanguage. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::UtilityLabels**](UtilityLabels.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **languages_get**
> ::models::Languages languages_get(app_id, app_key, optional)
Lists available dictionaries

Returns a list of monolingual and bilingual language datasets available in the API 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
 **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **source_language** | **String**| IANA language code. If provided output will be filtered by sourceLanguage. | 
 **target_language** | **String**| IANA language code. If provided output will be filtered by sourceLanguage. | 

### Return type

[**::models::Languages**](Languages.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **lexicalcategories_language_get**
> ::models::UtilityLabels lexicalcategories_language_get(language, app_id, app_key)
Lists available lexical categories in a dataset

Returns a list of available [lexical categories](documentation/glossary?term=lexicalcategory) for a given language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::UtilityLabels**](UtilityLabels.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **regions_source_language_get**
> ::models::Regions regions_source_language_get(source_language, app_id, app_key)
Lists available regions in a monolingual dataset

Returns a list of the available [regions](documentation/glossary?term=regions) for a given monolingual language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Regions**](Regions.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **registers_source_language_get**
> ::models::UtilityLabels registers_source_language_get(source_language, app_id, app_key)
Lists available registers in a  monolingual dataset

Returns a list of the available [registers](documentation/glossary?term=registers) for a given monolingual language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::UtilityLabels**](UtilityLabels.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **registers_source_register_language_target_register_language_get**
> ::models::UtilityLabels registers_source_register_language_target_register_language_get(source_register_language, target_register_language, app_id, app_key)
Lists available registers in a bilingual dataset

Returns a list of the available [registers](documentation/glossary?term=registers) for a given bilingual language dataset. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_register_language** | **String**| IANA language code | 
  **target_register_language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::UtilityLabels**](UtilityLabels.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

