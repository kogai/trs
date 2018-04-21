# \SearchApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_source_lang_get**](SearchApi.md#search_source_lang_get) | **Get** /search/{source_lang} | Retrieve possible matches to input
[**search_source_search_language_translationstarget_search_language_get**](SearchApi.md#search_source_search_language_translationstarget_search_language_get) | **Get** /search/{source_search_language}/translations&#x3D;{target_search_language} | Retrieve possible translation matches to input


# **search_source_lang_get**
> ::models::Wordlist search_source_lang_get(source_lang, app_id, app_key, optional)
Retrieve possible matches to input

 Use this to retrieve possible [headword](documentation/glossary?term=headword) matches for a given string of text. The results are culculated using headword matching, fuzzy matching, and [lemmatization](documentation/glossary?term=lemma)     <div id=\"search\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source_lang** | **String**| IANA language code | 
 **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
 **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **q** | **String**| Search string | [default to eye]
 **prefix** | **bool**| Set prefix to true if you&#39;d like to get results only starting with search string. | [default to false]
 **regions** | **String**| If searching in English, filter words with specific region(s) either &#39;us&#39; or &#39;gb&#39;. | 
 **limit** | **String**| Limit the number of results per response. Default and maximum limit is 5000. | 
 **offset** | **String**| Offset the start number of the result. | 

### Return type

[**::models::Wordlist**](Wordlist.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **search_source_search_language_translationstarget_search_language_get**
> ::models::Wordlist search_source_search_language_translationstarget_search_language_get(source_search_language, target_search_language, app_id, app_key, optional)
Retrieve possible translation matches to input

 Use this to find matches in our translation dictionaries.    <div id=\"search_translation\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_search_language** | **String**| IANA language code | 
  **target_search_language** | **String**| IANA language code | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source_search_language** | **String**| IANA language code | 
 **target_search_language** | **String**| IANA language code | 
 **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
 **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **q** | **String**| Search string. | [default to eye]
 **prefix** | **bool**| Set prefix to true if you&#39;d like to get results only starting with search string. | [default to false]
 **regions** | **String**| Filter words with specific region(s) E.g., regions&#x3D;us. For now gb, us are available for en language. | 
 **limit** | **String**| Limit the number of results per response. Default and maximum limit is 5000. | 
 **offset** | **String**| Offset the start number of the result. | 

### Return type

[**::models::Wordlist**](Wordlist.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

