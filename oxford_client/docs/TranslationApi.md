# \TranslationApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entries_source_translation_language_word_id_translationstarget_translation_language_get**](TranslationApi.md#entries_source_translation_language_word_id_translationstarget_translation_language_get) | **Get** /entries/{source_translation_language}/{word_id}/translations&#x3D;{target_translation_language} | Retrieve translation for a given word


# **entries_source_translation_language_word_id_translationstarget_translation_language_get**
> ::models::RetrieveEntry entries_source_translation_language_word_id_translationstarget_translation_language_get(source_translation_language, word_id, target_translation_language, app_id, app_key)
Retrieve translation for a given word

 Use this to return translations for a given word. In the event that a word in the dataset does not have a direct translation, the response will be a [definition](documentation/glossary?term=entry) in the target language.    <div id=\"translation\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_translation_language** | **String**| IANA language code | 
  **word_id** | **String**| The source word | 
  **target_translation_language** | **String**| IANA language code | 
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

