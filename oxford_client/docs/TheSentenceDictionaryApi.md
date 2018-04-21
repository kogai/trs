# \TheSentenceDictionaryApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**entries_source_language_word_id_sentences_get**](TheSentenceDictionaryApi.md#entries_source_language_word_id_sentences_get) | **Get** /entries/{source_language}/{word_id}/sentences | Retrieve corpus sentences for a given word


# **entries_source_language_word_id_sentences_get**
> ::models::SentencesResults entries_source_language_word_id_sentences_get(source_language, word_id, app_id, app_key)
Retrieve corpus sentences for a given word

 Use this to retrieve sentences extracted from  corpora which show how a word is used in the language. This is available for English and Spanish. For English, the sentences are linked to the correct [sense](documentation/glossary?term=sense) of the word in the dictionary. In Spanish, they are linked at the [headword](documentation/glossary?term=headword) level.   <div id=\"sentences\"></div> 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_language** | **String**| IANA language code | 
  **word_id** | **String**| An Entry identifier. Case-sensitive. | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]

### Return type

[**::models::SentencesResults**](SentencesResults.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

