# StatsWordResultResult

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**frequency** | **i32** | The number of times a word appears in the entire corpus | [default to null]
**lemma** | **String** | A lemma of the word (e.g., wordforms \&quot;lay\&quot;, \&quot;laid\&quot; and \&quot;laying\&quot; have all lemma \&quot;lay\&quot;) | [optional] [default to null]
**lexical_category** | **String** | A lexical category such as &#39;verb&#39; or &#39;noun&#39; | [optional] [default to null]
**match_count** | **i32** | The number of database records that matched the query params (stated frequency is the sum of the individual frequencies) | [default to null]
**normalized_frequency** | **i32** | The number of times a word appears on average in 1 million words | [default to null]
**true_case** | **String** | A given written realisation of a an entry (e.g., \&quot;lay\&quot;) usually lower case | [optional] [default to null]
**wordform** | **String** | A given written realisation of a an entry (e.g., \&quot;Lay\&quot;) preserving case | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


