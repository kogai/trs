# \LexiStatsApi

All URIs are relative to *https://od-api-demo.oxforddictionaries.com:443/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stats_frequency_ngrams_source_lang_corpus_ngram_size_get**](LexiStatsApi.md#stats_frequency_ngrams_source_lang_corpus_ngram_size_get) | **Get** /stats/frequency/ngrams/{source_lang}/{corpus}/{ngram-size}/ | Retrieve the frequency of ngrams (1-4) derived from a corpus
[**stats_frequency_word_source_lang_get**](LexiStatsApi.md#stats_frequency_word_source_lang_get) | **Get** /stats/frequency/word/{source_lang}/ | Retrieve the frequency of a word derived from a corpus.
[**stats_frequency_words_source_lang_get**](LexiStatsApi.md#stats_frequency_words_source_lang_get) | **Get** /stats/frequency/words/{source_lang}/ | Retrieve a list of frequencies of a word/words derived from a corpus.


# **stats_frequency_ngrams_source_lang_corpus_ngram_size_get**
> ::models::NgramsResult stats_frequency_ngrams_source_lang_corpus_ngram_size_get(source_lang, corpus, ngram_size, app_id, app_key, optional)
Retrieve the frequency of ngrams (1-4) derived from a corpus

This endpoint returns frequencies of ngrams of size 1-4. That is the number of times a word (ngram size = 1) or words (ngram size > 1) appear in the corpus. Ngrams are case sensitive (\"I AM\" and \"I am\" will have different frequency) and frequencies are calculated per word (true case) so \"the book\" and \"the books\" are two different ngrams. The results can be filtered based on query parameters. <br> <br> Parameters can be provided in PATH, GET or POST (form or json). The parameters in PATH are overridden by parameters in GET, POST and json (in that order). In PATH, individual options are separated by semicolon and values are separated by commas (where multiple values can be used). <br> <br> Example for bigrams (ngram of size 2): * PATH: /tokens=a word,another word * GET: /?tokens=a word&tokens=another word * POST (json):    ```javascript     {         \"tokens\": [\"a word\", \"another word\"]     }   ```  Either \"tokens\" or \"contains\" has to be provided. <br> <br> Some queries with \"contains\" or \"sort\" can exceed the 30s timeout, in which case the API will return an error message with status code 503. You mitigate this by providing additional restrictions such as \"minFrequency\" and \"maxFrequency\". <br> <br> You can use the parameters \"offset\" and \"limit\" to paginate through large result sets. For convenience, the HTTP header \"Link\" is set on the response to provide links to \"first\", \"self\", \"next\", \"prev\" and \"last\" pages of results (depending on the context). For example, if the result set contains 50 results and the parameter \"limit\" is set to 25, the Links header will contain an URL for the first 25 results and the next 25 results. <br> <br> Some libraries such as python's `requests` can parse the header automatically and offer a convenient way of iterating through the results. For example: ```python def get_all_results(url):     while url:         r = requests.get(url)         r.raise_for_status()         for item in r.json()['results']:           yield item         url = r.links.get('next', {}).get('url') ``` 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **source_lang** | **String**| IANA language code | 
  **corpus** | **String**| For corpora other than &#39;nmc&#39; (New Monitor Corpus) please contact api@oxforddictionaries.com | 
  **ngram_size** | **String**| the size of ngrams requested (1-4) | 
  **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
  **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **source_lang** | **String**| IANA language code | 
 **corpus** | **String**| For corpora other than &#39;nmc&#39; (New Monitor Corpus) please contact api@oxforddictionaries.com | 
 **ngram_size** | **String**| the size of ngrams requested (1-4) | 
 **app_id** | **String**| App ID Authentication Parameter | [default to 5037d509]
 **app_key** | **String**| App Key Authentication Parameter | [default to 4dc1aebaa63721f0f8e79a55e2514bc7]
 **tokens** | **String**| List of tokens to filter. The tokens are separated by spaces, the list items are separated by comma (e.g., for bigrams (n&#x3D;2) tokens&#x3D;this is,this was, this will) | [default to a word]
 **contains** | **String**| Find ngrams containing the given token(s). Use comma or space as token separators; the order of tokens is irrelevant. | 
 **punctuation** | **String**| Flag specifying whether to lookup ngrams that include punctuation or not (possible values are \&quot;true\&quot; and \&quot;false\&quot;; default is \&quot;false\&quot;) | 
 **format** | **String**| Option specifying whether tokens should be returned as a single string (option \&quot;google\&quot;) or as a list of strings (option \&quot;oup\&quot;) | [default to oup]
 **min_frequency** | **i64**| Restrict the query to entries with frequency of at least &#x60;minFrequency&#x60; | 
 **max_frequency** | **i64**| Restrict the query to entries with frequency of at most &#x60;maxFrequency&#x60; | 
 **min_document_frequency** | **i64**| Restrict the query to entries that appear in at least &#x60;minDocumentFrequency&#x60; documents | 
 **max_document_frequency** | **i64**| Restrict the query to entries that appera in at most &#x60;maxDocumentFrequency&#x60; documents | 
 **collate** | **String**| collate the results by wordform, trueCase, lemma, lexicalCategory. Multiple values can be separated by commas (e.g., collate&#x3D;trueCase,lemma,lexicalCategory). | 
 **sort** | **String**| sort the resulting list by wordform, trueCase, lemma, lexicalCategory, frequency, normalizedFrequency. Descending order is achieved by prepending the value with the minus sign (&#39;-&#39;). Multiple values can be separated by commas (e.g., sort&#x3D;lexicalCategory,-frequency) | 
 **offset** | **i64**| pagination - results offset | [default to 0]
 **limit** | **i64**| pagination - results limit | [default to 100]

### Return type

[**::models::NgramsResult**](NgramsResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **stats_frequency_word_source_lang_get**
> ::models::StatsWordResult stats_frequency_word_source_lang_get(source_lang, app_id, app_key, optional)
Retrieve the frequency of a word derived from a corpus.

This endpoint provides the frequency of a given word. When multiple database records match the query parameters, the returned frequency is the sum of the individual frequencies. For example, if the query parameters are lemma=test, the returned frequency will include the verb \"test\", the noun \"test\" and the adjective \"test\" in all forms (Test, tested, testing, etc.) <br> <br> If you are interested in the frequency of the word \"test\" but want to exclude other forms (e.g., tested) use the option trueCase=test. Normally, the word \"test\" will be spelt with a capital letter at the beginning of a sentence. The option trueCase will ignore this and it will count \"Test\" and \"test\" as the same token. If you are interested in frequencies of \"Test\" and \"test\", use the option wordform=test or wordform=Test. Note that trueCase is not just a lower case of the word as some words are genuinely spelt with a capital letter such as the word \"press\" in Oxford University Press. <br> <br> Parameters can be provided in PATH, GET or POST (form or json). The parameters in PATH are overriden by parameters in GET, POST and json (in that order). In PATH, individual options are separated by semicolon and values are separated by commas (where multiple values can be used). Examples: * PATH: /lemma=test;lexicalCategory=noun * GET: /?lemma=test&lexicalCategory=noun * POST (json):    ```javascript     {       \"lemma\": \"test\",       \"lexicalCategory\": \"noun\"     }   ```  <br> One of the options wordform/trueCase/lemma/lexicalCategory has to be provided. 

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
 **corpus** | **String**| For corpora other than &#39;nmc&#39; (New Monitor Corpus) please contact api@oxforddictionaries.com | [default to nmc]
 **wordform** | **String**| The written form of the word to look up (preserving case e.g., Books vs books) | 
 **true_case** | **String**| The written form of the word to look up with normalised case (Books --&gt; books) | 
 **lemma** | **String**| The lemma of the word to look up (e.g., Book, booked, books all have the lemma \&quot;book\&quot;) | [default to test]
 **lexical_category** | **String**| The lexical category of the word(s) to look up (e.g., noun or verb) | 

### Return type

[**::models::StatsWordResult**](StatsWordResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **stats_frequency_words_source_lang_get**
> ::models::StatsWordResultList stats_frequency_words_source_lang_get(source_lang, app_id, app_key, optional)
Retrieve a list of frequencies of a word/words derived from a corpus.

This endpoint provides a list of frequencies for a given word or words. Unlike the /word/ endpoint, the results are split into the smallest units. <br> <br> To exclude a specific value, prepend it with the minus sign ('-'). For example, to get frequencies of the lemma 'happy' but exclude superlative forms (i.e., happiest) you could use options 'lemma=happy;grammaticalFeatures=-degreeType:superlative'. <br> <br> Parameters can be provided in PATH, GET or POST (form or json). The parameters in PATH are overridden by parameters in GET, POST and json (in that order). In PATH, individual options are separated by semicolon and values are separated by commas (where multiple values can be used). <br> <br> The parameters wordform/trueCase/lemma/lexicalCategory also exist in a plural form, taking a lists of items. Examples: * PATH: /wordforms=happy,happier,happiest * GET: /?wordforms=happy&wordforms=happier&wordforms=happiest * POST (json): ```javascript   {     \"wordforms\": [\"happy\", \"happier\", \"happiest\"]   } ``` A mor complex example of retrieving frequencies of multiple lemmas: ```   {       \"lemmas\": [\"happy\", \"content\", \"cheerful\", \"cheery\", \"merry\", \"joyful\", \"ecstatic\"],       \"grammaticalFeatures\": {           \"adjectiveFunctionType\": \"predicative\"       },       \"lexicalCategory\": \"adjective\",       \"sort\": [\"lemma\", \"-frequency\"]   } ``` Some queries with \"collate\" or \"sort\" can exceed the 30s timeout, in which case the API will return an error message with status code 503. You mitigate this by providing additional restrictions such as \"minFrequency\" and \"maxFrequency\". <br> <br> You can use the parameters \"offset\" and \"limit\" to paginate through large result sets. For convenience, the HTTP header \"Link\" is set on the response to provide links to \"first\", \"self\", \"next\", \"prev\" and \"last\" pages of results (depending on the context). For example, if the result set contains 50 results and the parameter \"limit\" is set to 25, the Links header will contain an URL for the first 25 results and the next 25 results. <br> <br> Some libraries such as python's `requests` can parse the header automatically and offer a convenient way of iterating through the results. For example: ```python def get_all_results(url):     while url:         r = requests.get(url)         r.raise_for_status()         for item in r.json()['results']:           yield item         url = r.links.get('next', {}).get('url') ``` 

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
 **corpus** | **String**| For corpora other than &#39;nmc&#39; (New Monitor Corpus) please contact api@oxforddictionaries.com | [default to nmc]
 **wordform** | **String**| The written form of the word to look up (preserving case e.g., Book vs book) | 
 **true_case** | **String**| The written form of the word to look up with normalised case (Books --&gt; books) | 
 **lemma** | **String**| The lemma of the word to look up (e.g., Book, booked, books all have the lemma \&quot;book\&quot;) | [default to test]
 **lexical_category** | **String**| The lexical category of the word(s) to look up (e.g., adjective or noun) | 
 **grammatical_features** | **String**| The grammatical features of the word(s) to look up entered as a list of k:v (e.g., degree_type:comparative) | 
 **sort** | **String**| sort the resulting list by wordform, trueCase, lemma, lexicalCategory, frequency, normalizedFrequency. Descending order is achieved by prepending the value with the minus sign (&#39;-&#39;). Multiple values can be separated by commas (e.g., sort&#x3D;lexicalCategory,-frequency) | 
 **collate** | **String**| collate the results by wordform, trueCase, lemma, lexicalCategory. Multiple values can be separated by commas (e.g., collate&#x3D;trueCase,lemma,lexicalCategory). | 
 **min_frequency** | **i64**| Restrict the query to entries with frequency of at least &#x60;minFrequency&#x60; | 
 **max_frequency** | **i64**| Restrict the query to entries with frequency of at most &#x60;maxFrequency&#x60; | 
 **min_normalized_frequency** | **f32**| Restrict the query to entries with frequency of at least &#x60;minNormalizedFrequency&#x60; | 
 **max_normalized_frequency** | **f32**| Restrict the query to entries with frequency of at most &#x60;maxNormalizedFrequency&#x60; | 
 **offset** | **i64**| pagination - results offset | [default to 0]
 **limit** | **i64**| pagination - results limit | [default to 100]

### Return type

[**::models::StatsWordResultList**](StatsWordResultList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

