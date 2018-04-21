# \LemmatronApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inflections_source_lang_word_id_filters_get**](LemmatronApi.md#inflections_source_lang_word_id_filters_get) | **Get** /inflections/{source_lang}/{word_id}/{filters} | Apply optional filters to Lemmatron response
[**inflections_source_lang_word_id_get**](LemmatronApi.md#inflections_source_lang_word_id_get) | **Get** /inflections/{source_lang}/{word_id} | Check a word exists in the dictionary and retrieve its root form


# **inflections_source_lang_word_id_filters_get**
> ::models::Lemmatron inflections_source_lang_word_id_filters_get(source_lang, word_id, filters, app_id, app_key)
Apply optional filters to Lemmatron response

 Retrieve available [lemmas](documentation/glossary?term=lemma) for a given [inflected](documentation/glossary?term=inflection) wordform. Filter results by categories.      <div id=\"lemmatron_filters\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| The input word | 
  **filters** | [**Vec&lt;String&gt;**](String.md)| Separate filtering conditions using a semicolon. Conditions take values grammaticalFeatures and/or lexicalCategory and are case-sensitive. To list multiple values in single condition divide them with comma. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Lemmatron**](Lemmatron.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **inflections_source_lang_word_id_get**
> ::models::Lemmatron inflections_source_lang_word_id_get(source_lang, word_id, app_id, app_key)
Check a word exists in the dictionary and retrieve its root form

 Use this to check if a word exists in the dictionary, or what 'root' form it links to (e.g., swimming > swim). The response tells you the possible [lemmas](documentation/glossary?term=lemma) for a given [inflected](documentation/glossary?term=inflection) word. This can then be combined with other endpoints to retrieve more information.    <div id=\"lemmatron\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| The input word | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Lemmatron**](Lemmatron.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

