# Sense

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cross_reference_markers** | [***::models::Arrayofstrings**](arrayofstrings.md) | A grouping of crossreference notes. | [optional] [default to null]
**cross_references** | [***::models::CrossReferencesList**](CrossReferencesList.md) |  | [optional] [default to null]
**definitions** | [***::models::Arrayofstrings**](arrayofstrings.md) | A list of statements of the exact meaning of a word | [optional] [default to null]
**domains** | [***::models::Arrayofstrings**](arrayofstrings.md) | A subject, discipline, or branch of knowledge particular to the Sense | [optional] [default to null]
**examples** | [***::models::ExamplesList**](ExamplesList.md) |  | [optional] [default to null]
**id** | **String** | The id of the sense that is required for the delete procedure | [optional] [default to null]
**notes** | [***::models::CategorizedTextList**](CategorizedTextList.md) |  | [optional] [default to null]
**pronunciations** | [***::models::PronunciationsList**](PronunciationsList.md) |  | [optional] [default to null]
**regions** | [***::models::Arrayofstrings**](arrayofstrings.md) | A particular area in which the Sense occurs, e.g. &#39;Great Britain&#39; | [optional] [default to null]
**registers** | [***::models::Arrayofstrings**](arrayofstrings.md) | A level of language usage, typically with respect to formality. e.g. &#39;offensive&#39;, &#39;informal&#39; | [optional] [default to null]
**short_definitions** | [***::models::Arrayofstrings**](arrayofstrings.md) | A list of short statements of the exact meaning of a word | [optional] [default to null]
**subsenses** | [**Vec<::models::Sense>**](Sense.md) | Ordered list of subsenses of a sense | [optional] [default to null]
**thesaurus_links** | [**Vec<::models::ThesaurusLink>**](thesaurusLink.md) | Ordered list of links to the Thesaurus Dictionary | [optional] [default to null]
**translations** | [***::models::TranslationsList**](TranslationsList.md) |  | [optional] [default to null]
**variant_forms** | [***::models::VariantFormsList**](VariantFormsList.md) | Various words that are used interchangeably depending on the context, e.g &#39;duck&#39; and &#39;duck boat&#39; | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


