# \ThesaurusApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entries_source_lang_word_id_antonyms_get**](ThesaurusApi.md#entries_source_lang_word_id_antonyms_get) | **Get** /entries/{source_lang}/{word_id}/antonyms | Retrieve words that mean the opposite
[**entries_source_lang_word_id_synonyms_get**](ThesaurusApi.md#entries_source_lang_word_id_synonyms_get) | **Get** /entries/{source_lang}/{word_id}/synonyms | Retrieve words that are similar
[**entries_source_lang_word_id_synonymsantonyms_get**](ThesaurusApi.md#entries_source_lang_word_id_synonymsantonyms_get) | **Get** /entries/{source_lang}/{word_id}/synonyms;antonyms | Retrieve synonyms and antonyms for a given word


# **entries_source_lang_word_id_antonyms_get**
> ::models::Thesaurus entries_source_lang_word_id_antonyms_get(source_lang, word_id, app_id, app_key)
Retrieve words that mean the opposite

 Retrieve words that are opposite in meaning to the input word ([antonym](documentation/glossary?term=thesaurus)).    <div id=\"antonyms\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Thesaurus**](Thesaurus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **entries_source_lang_word_id_synonyms_get**
> ::models::Thesaurus entries_source_lang_word_id_synonyms_get(source_lang, word_id, app_id, app_key)
Retrieve words that are similar

 Use this to retrieve words that are similar in meaning to the input word ([synonym](documentation/glossary?term=synonym)).    <div id=\"synonyms\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Thesaurus**](Thesaurus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **entries_source_lang_word_id_synonymsantonyms_get**
> ::models::Thesaurus entries_source_lang_word_id_synonymsantonyms_get(source_lang, word_id, app_id, app_key)
Retrieve synonyms and antonyms for a given word

 Retrieve available [synonyms](documentation/glossary?term=thesaurus) and [antonyms](documentation/glossary?term=thesaurus) for a given word and language.     <div id=\"synonyms_and_antonyms\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::Thesaurus**](Thesaurus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

