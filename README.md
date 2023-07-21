# Rust fluent codes

This library can be use to generate fluent codes.

## Example

To generate code with four words:

 ```rust
 use fluent_codes_rust::FluentCodes;
println!("{}", FluentCodes::generate_code_with_four_words())
 ```

 ```text
 Output: fluffy-vacuum-misuse-deadly
 ```

To generate code with three words and six digits:

 ```rust
 use fluent_codes_rust::FluentCodes;
println!("{}", FluentCodes::generate_code_with_three_words_and_six_digits())
 ```

 ```text
 Output: calmer-taints-fourty-887709
 ```

Or you can use builder methods:

 ```rust
 use fluent_codes_rust::FluentCodes;
println!("{}", FluentCodes::default()
    .with_min_length(3).with_max_length(8)
    .with_joiner("..{-_-}..".to_string())
    .adjective().adposition().adverb()
    .auxiliary().coordinating_conjunction().determiner().interjection()
    .noun().particle().pronoun().proper_noun()
    .punctuation().subordinating_conjunction().symbol().verb().six_digits()
    .to_string())
 ```

 ```text
 Output: fused..{-_-}..jpg..{-_-}..reliably..{-_-}..lolcat..{-_-}..jdlugosz..{-_-}..resarted..{-_-}..878533
 ```

### Words

Words are generated using code @ https://github.com/sam-mmm/word_generator

Definitions of terms adjective, adposition, adverb, auxiliary, coordinatingConjunction, determiner, interjection, noun,
numeral, particle, pronoun, properNoun, punctuation, subordinatingConjunction, symbol, verb
are from https://universaldependencies.org/u/pos/

### License

http://www.apache.org/licenses/LICENSE-2.0