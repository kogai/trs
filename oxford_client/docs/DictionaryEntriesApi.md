# \DictionaryEntriesApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entries_source_lang_word_id_filters_get**](DictionaryEntriesApi.md#entries_source_lang_word_id_filters_get) | **Get** /entries/{source_lang}/{word_id}/{filters} | Apply filters to response
[**entries_source_lang_word_id_get**](DictionaryEntriesApi.md#entries_source_lang_word_id_get) | **Get** /entries/{source_lang}/{word_id} | Retrieve dictionary information for a given word
[**entries_source_lang_word_id_regionsregion_get**](DictionaryEntriesApi.md#entries_source_lang_word_id_regionsregion_get) | **Get** /entries/{source_lang}/{word_id}/regions&#x3D;{region} | Specify GB or US dictionary for English entry search


# **entries_source_lang_word_id_filters_get**
> ::models::RetrieveEntry entries_source_lang_word_id_filters_get(source_lang, word_id, filters, app_id, app_key)
Apply filters to response

 Use filters to limit the [entry](documentation/glossary?term=entry) information that is returned. For example, you may only require definitions and not everything else, or just [pronunciations](documentation/glossary?term=pronunciation). The full list of filters can be retrieved from the filters Utility endpoint. You can also specify values within the filter using '='. For example 'grammaticalFeatures=singular'. Filters can also be combined using a semicolon.    <div id=\"dictionary_entries_filters\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **filters** | [**Vec&lt;String&gt;**](String.md)| Separate filtering conditions using a semicolon. Conditions take values grammaticalFeatures and/or lexicalCategory and are case-sensitive. To list multiple values in single condition divide them with comma. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::RetrieveEntry**](RetrieveEntry.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **entries_source_lang_word_id_get**
> ::models::RetrieveEntry entries_source_lang_word_id_get(source_lang, word_id, app_id, app_key)
Retrieve dictionary information for a given word

 Use this to retrieve definitions, [pronunciations](documentation/glossary?term=pronunciation), example sentences, [grammatical information](documentation/glossary?term=grammaticalfeatures) and [word origins](documentation/glossary?term=etymology). It only works for dictionary [headwords](documentation/glossary?term=headword), so you may need to use the [Lemmatron](documentation/glossary?term=lemma) first if your input is likely to be an [inflected](documentation/glossary?term=inflection) form (e.g., 'swimming'). This would return the linked [headword](documentation/glossary?term=headword) (e.g., 'swim') which you can then use in the Entries endpoint. Unless specified using a region filter, the default lookup will be the Oxford Dictionary of English (GB).    <div id=\"dictionary_entries\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::RetrieveEntry**](RetrieveEntry.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **entries_source_lang_word_id_regionsregion_get**
> ::models::RetrieveEntry entries_source_lang_word_id_regionsregion_get(source_lang, word_id, region, app_id, app_key)
Specify GB or US dictionary for English entry search

 USe this filter to restrict the lookup to either our Oxford Dictionary of English (GB) or New Oxford American Dictionary (US). 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **region** | **String**| Region filter parameter. gb &#x3D; Oxford Dictionary of English. us &#x3D; New Oxford American Dictionary. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::RetrieveEntry**](RetrieveEntry.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

