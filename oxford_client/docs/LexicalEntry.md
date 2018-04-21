# LexicalEntry

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**derivative_of** | [***::models::ArrayOfRelatedEntries**](ArrayOfRelatedEntries.md) | Other words from which this one derives | [optional] [default to null]
**derivatives** | [***::models::ArrayOfRelatedEntries**](ArrayOfRelatedEntries.md) | Other words from which their Sense derives | [optional] [default to null]
**entries** | [**Vec<::models::Entry>**](Entry.md) |  | [optional] [default to null]
**grammatical_features** | [***::models::GrammaticalFeaturesList**](GrammaticalFeaturesList.md) |  | [optional] [default to null]
**language** | **String** | IANA language code | [default to null]
**lexical_category** | **String** | A linguistic category of words (or more precisely lexical items), generally defined by the syntactic or morphological behaviour of the lexical item in question, such as noun or verb | [default to null]
**notes** | [***::models::CategorizedTextList**](CategorizedTextList.md) |  | [optional] [default to null]
**pronunciations** | [***::models::PronunciationsList**](PronunciationsList.md) |  | [optional] [default to null]
**text** | **String** | A given written or spoken realisation of a an entry. | [default to null]
**variant_forms** | [***::models::VariantFormsList**](VariantFormsList.md) | Various words that are used interchangeably depending on the context, e.g &#39;a&#39; and &#39;an&#39; | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


