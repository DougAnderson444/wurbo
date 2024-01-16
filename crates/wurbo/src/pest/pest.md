# Syntax of pest grammars

`pest` grammars are lists of rules. Rules are defined like this:

```pest
//! Grammar doc
my_rule = { ... }

/// Rule doc
another_rule = {        // comments are preceded by two slashes
    ...                 // whitespace goes anywhere
}
```

Since rule names are translated into Rust enum variants, they are not allowed
to be Rust keywords.

The left curly bracket `{` defining a rule can be preceded by [symbols that
affect its operation]:

```pest
silent_rule = _{ ... }
atomic_rule = @{ ... }
```

[symbols that affect its operation]: #silent-and-atomic-rules

## Expressions

Grammar rules are built from _expressions_ (hence "parsing expression
grammar"). These expressions are a terse, formal description of how to parse an
input string.

Expressions are composable: they can be built out of other expressions and
nested inside of each other to produce arbitrarily complex rules (although you
should break very complicated expressions into multiple rules to make them
easier to manage).

PEG expressions are suitable for both high-level meaning, like "a function
signature, followed by a function body", and low-level meaning, like "a
semicolon, followed by a line feed". The combining form "followed by",
the [sequence operator], is the same in either case.

[sequence operator]: #sequence

### Terminals

The most basic rule is a **literal string** in double quotes: `"text"`.

A string can be **case-insensitive** (for ASCII characters only) if preceded by
a caret: `^"text"`.

A single **character in a range** is written as two single-quoted characters,
separated by two dots: `'0'..'9'`.

You can match **any single character** at all with the special rule `ANY`. This
is equivalent to `'\u{00}'..'\u{10FFFF}'`, any single Unicode character.

```
"a literal string"
^"ASCII case-insensitive string"
'a'..'z'
ANY
```

Finally, you can **refer to other rules** by writing their names directly, and
even **use rules recursively**:

```pest
my_rule = { "slithy " ~ other_rule }
other_rule = { "toves" }
recursive_rule = { "mimsy " ~ recursive_rule }
```

### Sequence

The sequence operator is written as a tilde `~`.

```
first ~ and_then

("abc") ~ (^"def") ~ ('g'..'z')        // matches "abcDEFr"
```

When matching a sequence expression, `first` is attempted. If `first` matches
successfully, `and_then` is attempted next. However, if `first` fails, the
entire expression fails.

A list of expressions can be chained together with sequences, which indicates
that _all_ of the components must occur, in the specified order.

### Ordered choice

The choice operator is written as a vertical line `|`.

```
first | or_else

("abc") | (^"def") | ('g'..'z')        // matches "DEF"
```

When matching a choice expression, `first` is attempted. If `first` matches
successfully, the entire expression _succeeds immediately_. However, if `first`
fails, `or_else` is attempted next.

Note that `first` and `or_else` are always attempted at the same position, even
if `first` matched some input before it failed. When encountering a parse
failure, the engine will try the next ordered choice as though no input had
been matched. Failed parses never consume any input.

```pest
start = { "Beware " ~ creature }
creature = {
    ("the " ~ "Jabberwock")
    | ("the " ~ "Jubjub bird")
}
```

```
"Beware the Jubjub bird"
 ^ (start) Parses via the second choice of `creature`,
           even though the first choice matched "the " successfully.
```

It is somewhat tempting to borrow terminology and think of this operation as
"alternation" or simply "OR", but this is misleading. The word "choice" is used
specifically because [the operation is *not* merely logical "OR"].

[the operation is *not* merely logical "OR"]: peg.html#ordered-choice

### Repetition

There are two repetition operators: the asterisk `*` and plus sign `+`. They
are placed after an expression. The asterisk `*` indicates that the preceding
expression can occur **zero or more** times. The plus sign `+` indicates that
the preceding expression can occur **one or more** times (it must occur at
least once).

The question mark operator `?` is similar, except it indicates that the
expression is **optional** &mdash; it can occur zero or one times.

```
("zero" ~ "or" ~ "more")*
 ("one" | "or" | "more")+
           (^"optional")?
```

Note that `expr*` and `expr?` will always succeed, because they are allowed to
match zero times. For example, `"a"* ~ "b"?` will succeed even on an empty
input string.

Other **numbers of repetitions** can be indicated using curly brackets:

```
expr{n}           // exactly n repetitions
expr{m, n}        // between m and n repetitions, inclusive

expr{, n}         // at most n repetitions
expr{m, }         // at least m repetitions
```

Thus `expr*` is equivalent to `expr{0, }`; `expr+` is equivalent to `expr{1,
}`; and `expr?` is equivalent to `expr{0, 1}`.

### Predicates

Preceding an expression with an ampersand `&` or exclamation mark `!` turns it
into a _predicate_ that never consumes any input. You might know these
operators as "lookahead" or "non-progressing".

The **positive predicate**, written as an ampersand `&`, attempts to match its
inner expression. If the inner expression succeeds, parsing continues, but at
the _same position_ as the predicate &mdash; `&foo ~ bar` is thus a kind of
"AND" statement: "the input string must match `foo` AND `bar`". If the inner
expression fails, the whole expression fails too.

The **negative predicate**, written as an exclamation mark `!`, attempts to
match its inner expression. If the inner expression _fails_, the predicate
_succeeds_ and parsing continues at the same position as the predicate. If the
inner expression _succeeds_, the predicate _fails_ &mdash; `!foo ~ bar` is thus
a kind of "NOT" statement: "the input string must match `bar` but NOT `foo`".

This leads to the common idiom meaning "any character but":

```pest
not_space_or_tab = {
    !(                // if the following text is not
        " "           //     a space
        | "\t"        //     or a tab
    )
    ~ ANY             // then consume one character
}

triple_quoted_string = {
    "'''"
    ~ triple_quoted_character*
    ~ "'''"
}
triple_quoted_character = {
    !"'''"        // if the following text is not three apostrophes
    ~ ANY         // then consume one character
}
```

## Operator precedence and grouping (WIP)

The repetition operators asterisk `*`, plus sign `+`, and question mark `?`
apply to the immediately preceding expression.

```
"One " ~ "or " ~ "more. "+
"One " ~ "or " ~ ("more. "+)
    are equivalent and match
"One or more. more. more. more. "
```

Larger expressions can be repeated by surrounding them with parentheses.

```
("One " ~ "or " ~ "more. ")+
    matches
"One or more. One or more. "
```

Repetition operators have the highest precedence, followed by predicate
operators, the sequence operator, and finally ordered choice.

```pest
my_rule = {
    "a"* ~ "b"?
    | &"b"+ ~ "a"
}

// equivalent to

my_rule = {
      ( ("a"*) ~ ("b"?) )
    | ( (&("b"+)) ~ "a" )
}
```

## Start and end of input

The rules `SOI` and `EOI` match the _start_ and _end_ of the input string,
respectively. Neither consumes any text. They only indicate whether the parser
is currently at one edge of the input.

For example, to ensure that a rule matches the entire input, where any syntax
error results in a failed parse (rather than a successful but incomplete
parse):

```pest
main = {
    SOI
    ~ (...)
    ~ EOI
}
```

## Implicit whitespace

Many languages and text formats allow arbitrary whitespace and comments between
logical tokens. For instance, Rust considers `4+5` equivalent to `4 + 5` and `4
/* comment */ + 5`.

The **optional rules `WHITESPACE` and `COMMENT`** implement this behaviour. If
either (or both) are defined, they will be implicitly inserted at every
[sequence] and between every [repetition] (except in [atomic rules]).

```pest
expression = { "4" ~ "+" ~ "5" }
WHITESPACE = _{ " " }
COMMENT = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
```

```
"4+5"
"4 + 5"
"4  +     5"
"4 /* comment */ + 5"
```

As you can see, `WHITESPACE` and `COMMENT` are run repeatedly, so they need
only match a single whitespace character or a single comment. The grammar above
is equivalent to:

```pest
expression = {
    "4"   ~ (ws | com)*
    ~ "+" ~ (ws | com)*
    ~ "5"
}
ws = _{ " " }
com = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
```

Note that Implicit whitespace is _not_ inserted at the beginning or end of rules
&mdash; for instance, `expression` does _not_ match `" 4+5 "`. If you want to
include Implicit whitespace at the beginning and end of a rule, you will need to
sandwich it between two empty rules (often `SOI` and `EOI` [as above]):

```pest
WHITESPACE = _{ " " }
expression = { "4" ~ "+" ~ "5" }
main = { SOI ~ expression ~ EOI }
```

```
"4+5"
"  4 + 5   "
```

(Be sure to mark the `WHITESPACE` and `COMMENT` rules as [silent] unless you
want to see them included inside other rules!)

[sequence]: #sequence
[repetition]: #repetition
[atomic rules]: #atomic
[as above]: #start-and-end-of-input
[silent]: #silent-and-atomic-rules

## Silent and atomic rules

### Silent

**Silent** rules are just like normal rules &mdash; when run, they function the
same way &mdash; except they do not produce [pairs] or [tokens]. If a rule is
silent, it will never appear in a parse result.

To make a silent rule, precede the left curly bracket `{` with a low line
(underscore) `_`.

```pest
silent = _{ ... }
```

Rules called from a silent rule are not treated as silent unless they are
declared to be silent. These rules may produce [pairs] or [tokens] and can appear
in a parse result.

[pairs]: ../parser_api.html#pairs
[tokens]: ../parser_api.html#tokens

### Atomic

Pest has two kinds of atomic rules: **atomic** and **compound atomic**. To
make one, write the sigil before the left curly bracket `{`.

```pest
/// Atomic rule start with `@`
atomic = @{ ... }

/// Compound Atomic start with `$`
compound_atomic = ${ ... }
```

Both kinds of atomic rule prevent [implicit whitespace]:

1. Inside an atomic rule, the tilde `~` means "immediately followed by".
2. [Repetition operators] (asterisk `*` and plus sign `+`) have no implicit separation.

In addition, all other rules called from an atomic rule are also treated as atomic.

The difference between the two is how they produce tokens for inner rules:

- **atomic** - In an Atomic rule, interior matching rules are [silent].
- **compound atomic** - By contrast, compound atomic rules produce inner tokens as normal.

Atomic rules are useful when the text you are parsing ignores whitespace except
in a few cases, such as literal strings. In this instance, you can write
`WHITESPACE` or `COMMENT` rules, then make your string-matching rule be atomic.

[implicit whitespace]: #implicit-whitespace
[repetition operators]: #repetition
[silent]: #silent-and-atomic-rules

### Non-atomic

Sometimes, you'll want to cancel the effects of atomic parsing. For instance,
you might want to have string interpolation with an expression inside, where
the inside expression can still have whitespace like normal.

```python
#!/bin/env python3
print(f"The answer is {2 + 4}.")
```

This is where you use a **non-atomic** rule. Write an exclamation mark `!` in
front of the defining curly bracket. The rule will run as non-atomic, whether
it is called from an atomic rule or not.

```pest
fstring = @{ "\"" ~ ... }
expr = !{ ... }
```

### Tags

Sometimes, you may want to attach a label to a part of a rule. This is useful for distinguishing
among different types of tokens (of the same expression) or for the ease of extracting information
from parse trees (without creating additional rules).
To do this, you can use the `#` symbol to bind a name to a part of a rule:

```pest
rule = { #tag = ... }
```

You can then access tags in your parse tree by using the `as_node_tag` method on `Pair`
or you can use the helper methods `find_first_tagged` or `find_tagged` on `Pairs`:

```rust
let pairs = ExampleParser::parse(Rule::example_rule, example_input).unwrap();
for pair in pairs.clone() {
    if let Some(tag) = pair.as_node_tag() {
        // ...
    }
}
let first = pairs.find_first_tagged("tag");
let all_tagged = pairs.find_tagged("tag");
```

Note that you need to enable "grammar-extras" feature to use this functionality:

```toml
# ...
pest_derive = { version = "2.7", features = ["grammar-extras"] }
```

## The stack (WIP)

`pest` maintains a stack that can be manipulated directly from the grammar. An
expression can be matched and pushed onto the stack with the keyword `PUSH`,
then later matched exactly with the keywords `PEEK` and `POP`.

Using the stack allows _the exact same text_ to be matched multiple times,
rather than _the same pattern_.

For example,

```pest
same_text = {
    PUSH( "a" | "b" | "c" )
    ~ POP
}
same_pattern = {
    ("a" | "b" | "c")
    ~ ("a" | "b" | "c")
}
```

In this case, `same_pattern` will match `"ab"`, while `same_text` will not.

One practical use is in parsing Rust ["raw string literals"], which look like
this:

```rust
const raw_str: &str = r###"
    Some number of number signs # followed by a quotation mark ".

    Quotation marks can be used anywhere inside: """"""""",
    as long as one is not followed by a matching number of number signs,
    which ends the string: "###;
```

When parsing a raw string, we have to keep track of how many number signs `#`
occurred before the quotation mark. We can do this using the stack:

```pest
raw_string = {
    "r" ~ PUSH("#"*) ~ "\""    // push the number signs onto the stack
    ~ raw_string_interior
    ~ "\"" ~ POP               // match a quotation mark and the number signs
}
raw_string_interior = {
    (
        !("\"" ~ PEEK)    // unless the next character is a quotation mark
                          // followed by the correct amount of number signs,
        ~ ANY             // consume one character
    )*
}
```

["raw string literals"]: https://doc.rust-lang.org/book/second-edition/appendix-02-operators.html#non-operator-symbols

### Indentation-Sensitive Languages

In conjunction with some extra helpers, the stack can be used to allow parsing indentation-sensitive languages, such as Python.

The general idea is that you store the leading whitespace on the stack with `PUSH` and then use `PEEK_ALL` to match _all_ of the whitespace on subsequent lines.

When exiting an indented block, use `DROP` to remove the stack entry without needing to match it.

An example grammar demonstrating this concept is given here:

```pest
Grammar = { SOI ~ NEWLINE* ~ BlockContent* ~ NEWLINE* ~ EOI }

NewBlock = _{
    // The first line in the block
    PEEK_ALL ~ PUSH("  "+ | "\t"+) ~ BlockContent ~
    // Subsequent lines in the block
    (PEEK_ALL ~ BlockContent)* ~
    // Remove the last layer of indentation from the stack when exiting the block
    DROP
}

BlockName = { ASCII_ALPHA+ }

BlockContent = {
    BlockName ~ (NEWLINE | EOI) ~ NewBlock*
}
```

This matches texts such as the following, whilst preserving indentation structure:

```
Hello
  This
    Is
    An
  Indentation
    Sensitive
      Language
Demonstration
```

# Cheat sheet

|      Syntax      |              Meaning              |         Syntax          |       Meaning        |
| :--------------: | :-------------------------------: | :---------------------: | :------------------: |
| `foo =  { ... }` |          [regular rule]           |    `baz = @{ ... }`     |       [atomic]       |
| `bar = _{ ... }` |             [silent]              |    `qux = ${ ... }`     |  [compound-atomic]   |
|   `#tag = ...`   |              [tags]               |   `plugh = !{ ... }`    |     [non-atomic]     |
|     `"abc"`      |          [exact string]           |        `^"abc"`         |  [case insensitive]  |
|    `'a'..'z'`    |         [character range]         |          `ANY`          |   [any character]    |
|   `foo ~ bar`    |            [sequence]             | <code>baz \| qux</code> |   [ordered choice]   |
|      `foo*`      |          [zero or more]           |         `bar+`          |    [one or more]     |
|      `baz?`      |            [optional]             |        `qux{n}`         |    [exactly *n*]     |
|   `qux{m, n}`    | [between *m* and *n* (inclusive)] |                         |                      |
|      `&foo`      |       [positive predicate]        |         `!bar`          | [negative predicate] |
|   `PUSH(baz)`    |         [match and push]          |                         |                      |
|      `POP`       |          [match and pop]          |         `PEEK`          | [match without pop]  |
|      `DROP`      |      [pop without matching]       |       `PEEK_ALL`        | [match entire stack] |

[regular rule]: #syntax-of-pest-parsers
[silent]: #silent-and-atomic-rules
[atomic]: #atomic
[compound-atomic]: #atomic
[non-atomic]: #non-atomic
[tags]: #tags
[exact string]: #terminals
[case insensitive]: #terminals
[character range]: #terminals
[any character]: #terminals
[sequence]: #sequence
[ordered choice]: #ordered-choice
[zero or more]: #repetition
[one or more]: #repetition
[optional]: #repetition
[exactly *n*]: #repetition
[between *m* and *n* (inclusive)]: #repetition
[positive predicate]: #predicates
[negative predicate]: #predicates
[match and push]: #the-stack-wip
[match and pop]: #the-stack-wip
[match without pop]: #the-stack-wip
[pop without matching]: #indentation-sensitive-languages
[match entire stack]: #indentation-sensitive-languages

# Pest Built-in rules

Besides `ANY`, matching any single Unicode character, `pest` provides several
rules to make parsing text more convenient.

## ASCII rules

Among the printable ASCII characters, it is often useful to match alphabetic
characters and numbers. For **numbers**, `pest` provides digits in common
radixes (bases):

|     Built-in rule     |                  Equivalent                   |
| :-------------------: | :-------------------------------------------: |
|     `ASCII_DIGIT`     |                  `'0'..'9'`                   |
| `ASCII_NONZERO_DIGIT` |                  `'1'..'9'`                   |
|   `ASCII_BIN_DIGIT`   |                  `'0'..'1'`                   |
|   `ASCII_OCT_DIGIT`   |                  `'0'..'7'`                   |
|   `ASCII_HEX_DIGIT`   | <code>'0'..'9' \| 'a'..'f' \| 'A'..'F'</code> |

For **alphabetic** characters, distinguishing between uppercase and lowercase:

|    Built-in rule    |            Equivalent             |
| :-----------------: | :-------------------------------: |
| `ASCII_ALPHA_LOWER` |            `'a'..'z'`             |
| `ASCII_ALPHA_UPPER` |            `'A'..'Z'`             |
|    `ASCII_ALPHA`    | <code>'a'..'z' \| 'A'..'Z'</code> |

And for **miscellaneous** use:

|    Built-in rule     |       Meaning        |               Equivalent                |
| :------------------: | :------------------: | :-------------------------------------: |
| `ASCII_ALPHANUMERIC` | any digit or letter  | <code>ASCII_DIGIT \| ASCII_ALPHA</code> |
|      `NEWLINE`       | any line feed format |   <code>"\n" \| "\r\n" \| "\r"</code>   |

## Unicode rules

To make it easier to correctly parse arbitrary Unicode text, `pest` includes a
large number of rules corresponding to Unicode character properties. These
rules are divided into **general category** and **binary property** rules.

Unicode characters are partitioned into categories based on their general
purpose. Every character belongs to a single category, in the same way that
every ASCII character is a control character, a digit, a letter, a symbol, or a
space.

In addition, every Unicode character has a list of binary properties (true or
false) that it does or does not satisfy. Characters can belong to any number of
these properties, depending on their meaning.

For example, the character "A", "Latin capital letter A", is in the general
category "Uppercase Letter" because its general purpose is being a letter. It
has the binary property "Uppercase" but not "Emoji". By contrast, the character
"&#x1F170;", "negative squared Latin capital letter A", is in the general
category "Other Symbol" because it does not generally occur as a letter in
text. It has both the binary properties "Uppercase" and "Emoji".

For more details, consult Chapter 4 of [The Unicode Standard].

[The Unicode Standard]: https://www.unicode.org/versions/latest/

### General categories

Formally, categories are non-overlapping: each Unicode character belongs to
exactly one category, and no category contains another. However, since certain
groups of categories are often useful together, `pest` exposes the hierarchy of
categories below. For example, the rule `CASED_LETTER` is not technically a
Unicode general category; it instead matches characters that are
`UPPERCASE_LETTER` or `LOWERCASE_LETTER`, which _are_ general categories.

- `LETTER`
  - `CASED_LETTER`
    - `UPPERCASE_LETTER`
    - `LOWERCASE_LETTER`
  - `TITLECASE_LETTER`
  - `MODIFIER_LETTER`
  - `OTHER_LETTER`
- `MARK`
  - `NONSPACING_MARK`
  - `SPACING_MARK`
  - `ENCLOSING_MARK`
- `NUMBER`
  - `DECIMAL_NUMBER`
  - `LETTER_NUMBER`
  - `OTHER_NUMBER`
- `PUNCTUATION`
  - `CONNECTOR_PUNCTUATION`
  - `DASH_PUNCTUATION`
  - `OPEN_PUNCTUATION`
  - `CLOSE_PUNCTUATION`
  - `INITIAL_PUNCTUATION`
  - `FINAL_PUNCTUATION`
  - `OTHER_PUNCTUATION`
- `SYMBOL`
  - `MATH_SYMBOL`
  - `CURRENCY_SYMBOL`
  - `MODIFIER_SYMBOL`
  - `OTHER_SYMBOL`
- `SEPARATOR`
  - `SPACE_SEPARATOR`
  - `LINE_SEPARATOR`
  - `PARAGRAPH_SEPARATOR`
- `OTHER`
  - `CONTROL`
  - `FORMAT`
  - `SURROGATE`
  - `PRIVATE_USE`
  - `UNASSIGNED`

### Binary properties

Many of these properties are used to define Unicode text algorithms, such as
[the bidirectional algorithm] and [the text segmentation algorithm]. Such
properties are not likely to be useful for most parsers.

However, the properties `XID_START` and `XID_CONTINUE` are particularly notable
because they are defined "to assist in the standard treatment of identifiers",
"such as programming language variables". See [Technical Report 31] for more
details.

[the bidirectional algorithm]: https://www.unicode.org/reports/tr9/
[the text segmentation algorithm]: https://www.unicode.org/reports/tr29/
[Technical Report 31]: https://www.unicode.org/reports/tr31/

- `ALPHABETIC`
- `BIDI_CONTROL`
- `BIDI_MIRRORED`
- `CASE_IGNORABLE`
- `CASED`
- `CHANGES_WHEN_CASEFOLDED`
- `CHANGES_WHEN_CASEMAPPED`
- `CHANGES_WHEN_LOWERCASED`
- `CHANGES_WHEN_TITLECASED`
- `CHANGES_WHEN_UPPERCASED`
- `DASH`
- `DEFAULT_IGNORABLE_CODE_POINT`
- `DEPRECATED`
- `DIACRITIC`
- `EMOJI`
- `EMOJI_COMPONENT`
- `EMOJI_MODIFIER`
- `EMOJI_MODIFIER_BASE`
- `EMOJI_PRESENTATION`
- `EXTENDED_PICTOGRAPHIC`
- `EXTENDER`
- `GRAPHEME_BASE`
- `GRAPHEME_EXTEND`
- `GRAPHEME_LINK`
- `HEX_DIGIT`
- `HYPHEN`
- `IDS_BINARY_OPERATOR`
- `IDS_TRINARY_OPERATOR`
- `ID_CONTINUE`
- `ID_START`
- `IDEOGRAPHIC`
- `JOIN_CONTROL`
- `LOGICAL_ORDER_EXCEPTION`
- `LOWERCASE`
- `MATH`
- `NONCHARACTER_CODE_POINT`
- `OTHER_ALPHABETIC`
- `OTHER_DEFAULT_IGNORABLE_CODE_POINT`
- `OTHER_GRAPHEME_EXTEND`
- `OTHER_ID_CONTINUE`
- `OTHER_ID_START`
- `OTHER_LOWERCASE`
- `OTHER_MATH`
- `OTHER_UPPERCASE`
- `PATTERN_SYNTAX`
- `PATTERN_WHITE_SPACE`
- `PREPENDED_CONCATENATION_MARK`
- `QUOTATION_MARK`
- `RADICAL`
- `REGIONAL_INDICATOR`
- `SENTENCE_TERMINAL`
- `SOFT_DOTTED`
- `TERMINAL_PUNCTUATION`
- `UNIFIED_IDEOGRAPH`
- `UPPERCASE`
- `VARIATION_SELECTOR`
- `WHITE_SPACE`
- `XID_CONTINUE`
- `XID_START`

### Script properties

The [Unicode script property](https://unicode.org/standard/supported.html)
has included built-in rules for matching characters in particular languages.

**For example:**

We want match a string that contains any CJK (regexp: `\p{CJK}`) characters such as `你好世界` or `こんにちは世界` or `안녕하세요 세계`.

- `HAN`: representing Chinese characters, including Simplified Chinese, Traditional Chinese, Japanese kanji, and Korean hanja.
- `HIRAGANA`: representing the Japanese hiragana syllabary.
- `KATAKANA`: representing the Japanese katakana syllabary.
- `HANGUL`: representing Korean alphabetical characters.
- `BOPOMOFO`: representing Chinese phonetic symbols.

So we define a rule named `CJK` like this:

```pest
CJK = { HAN | HIRAGANA | KATAKANA | HANGUL | BOPOMOFO }
```

**All available rules:**

- `ADLAM`
- `AHOM`
- `ANATOLIAN_HIEROGLYPHS`
- `ARABIC`
- `ARMENIAN`
- `AVESTAN`
- `BALINESE`
- `BAMUM`
- `BASSA_VAH`
- `BATAK`
- `BENGALI`
- `BHAIKSUKI`
- `BOPOMOFO`
- `BRAHMI`
- `BRAILLE`
- `BUGINESE`
- `BUHID`
- `CANADIAN_ABORIGINAL`
- `CARIAN`
- `CAUCASIAN_ALBANIAN`
- `CHAKMA`
- `CHAM`
- `CHEROKEE`
- `CHORASMIAN`
- `COMMON`
- `COPTIC`
- `CUNEIFORM`
- `CYPRIOT`
- `CYPRO_MINOAN`
- `CYRILLIC`
- `DESERET`
- `DEVANAGARI`
- `DIVES_AKURU`
- `DOGRA`
- `DUPLOYAN`
- `EGYPTIAN_HIEROGLYPHS`
- `ELBASAN`
- `ELYMAIC`
- `ETHIOPIC`
- `GEORGIAN`
- `GLAGOLITIC`
- `GOTHIC`
- `GRANTHA`
- `GREEK`
- `GUJARATI`
- `GUNJALA_GONDI`
- `GURMUKHI`
- `HAN`
- `HANGUL`
- `HANIFI_ROHINGYA`
- `HANUNOO`
- `HATRAN`
- `HEBREW`
- `HIRAGANA`
- `IMPERIAL_ARAMAIC`
- `INHERITED`
- `INSCRIPTIONAL_PAHLAVI`
- `INSCRIPTIONAL_PARTHIAN`
- `JAVANESE`
- `KAITHI`
- `KANNADA`
- `KATAKANA`
- `KAWI`
- `KAYAH_LI`
- `KHAROSHTHI`
- `KHITAN_SMALL_SCRIPT`
- `KHMER`
- `KHOJKI`
- `KHUDAWADI`
- `LAO`
- `LATIN`
- `LEPCHA`
- `LIMBU`
- `LINEAR_A`
- `LINEAR_B`
- `LISU`
- `LYCIAN`
- `LYDIAN`
- `MAHAJANI`
- `MAKASAR`
- `MALAYALAM`
- `MANDAIC`
- `MANICHAEAN`
- `MARCHEN`
- `MASARAM_GONDI`
- `MEDEFAIDRIN`
- `MEETEI_MAYEK`
- `MENDE_KIKAKUI`
- `MEROITIC_CURSIVE`
- `MEROITIC_HIEROGLYPHS`
- `MIAO`
- `MODI`
- `MONGOLIAN`
- `MRO`
- `MULTANI`
- `MYANMAR`
- `NABATAEAN`
- `NAG_MUNDARI`
- `NANDINAGARI`
- `NEW_TAI_LUE`
- `NEWA`
- `NKO`
- `NUSHU`
- `NYIAKENG_PUACHUE_HMONG`
- `OGHAM`
- `OL_CHIKI`
- `OLD_HUNGARIAN`
- `OLD_ITALIC`
- `OLD_NORTH_ARABIAN`
- `OLD_PERMIC`
- `OLD_PERSIAN`
- `OLD_SOGDIAN`
- `OLD_SOUTH_ARABIAN`
- `OLD_TURKIC`
- `OLD_UYGHUR`
- `ORIYA`
- `OSAGE`
- `OSMANYA`
- `PAHAWH_HMONG`
- `PALMYRENE`
- `PAU_CIN_HAU`
- `PHAGS_PA`
- `PHOENICIAN`
- `PSALTER_PAHLAVI`
- `REJANG`
- `RUNIC`
- `SAMARITAN`
- `SAURASHTRA`
- `SHARADA`
- `SHAVIAN`
- `SIDDHAM`
- `SIGNWRITING`
- `SINHALA`
- `SOGDIAN`
- `SORA_SOMPENG`
- `SOYOMBO`
- `SUNDANESE`
- `SYLOTI_NAGRI`
- `SYRIAC`
- `TAGALOG`
- `TAGBANWA`
- `TAI_LE`
- `TAI_THAM`
- `TAI_VIET`
- `TAKRI`
- `TAMIL`
- `TANGSA`
- `TANGUT`
- `TELUGU`
- `THAANA`
- `THAI`
- `TIBETAN`
- `TIFINAGH`
- `TIRHUTA`
- `TOTO`
- `UGARITIC`
- `VAI`
- `VITHKUQI`
- `WANCHO`
- `WARANG_CITI`
- `YEZIDI`
- `YI`
- `ZANABAZAR_SQUARE`
