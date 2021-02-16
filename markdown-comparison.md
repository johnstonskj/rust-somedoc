# Markdown Flavors

This is a brief document that describes the various flavors of Markdown compared to [John Gruber’s original design document](https://daringfireball.net/projects/markdown/syntax). Specifically call-outs will be made for the following:

1. [CommonMark](https://spec.commonmark.org/0.29/) (CM)
1. [GitHub Flavored Markdown](https://github.github.com/gfm/) (GFM)
1. [MultiMarkdown](https://fletcherpenney.net/multimarkdown/) (MMD)
1. [Markdown Extra (PHP)](https://michelf.ca/projects/php-markdown/extra/) (Extra)
1. [Pandoc](https://pandoc.org/MANUAL.html) (Pandoc)
1. [Kramdown](https://kramdown.gettalong.org/syntax.html) (Kramdown)

## Markdown Core

### Inline HTML

### Block Elements

#### Paragraphs & Line Breaks

#### Headers

> Adding IDs

#### BlockQuotes

#### Lists

#### Code Blocks

#### Horizontal Rules

### Span Elements

#### Links

#### Emphasis

#### Code

> Fenced code ``` or ~~~

#### Images

### Miscellaneous

#### Backslash Escapes

#### Automatic Links

## Full Extensions


## Comparison

| Group      | Feature                 | MD      | CM      | GFM     | MMD     | Extra   | Pandoc  | Kramdown |
| ---------- | ----------------------- | ------- | ------- | ------- | ------- | ------- | ------- | -------- |
| **HTML**   | Inline HTML             | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Fenced/Native div       | No      | No      | No      | No      | No      | **Yes** | No       |
|            | "[]" Anchors            | No      | No      | No      | **Yes** | No      | No      | No       |
|            | "{}" Special AL         | No      | No      | No      | No      | **Yes** | **Yes** | No       |
|            | "{:}" Inline AL         | No      | No      | No      | No      | No      | No      | **Yes**  |
| **Block**  | Paragraphs              | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Line Breaks             | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | "\" Line Breaks         | No      | No      | No      | **Yes** | No      | No      | No       |
|            | Headers - Setext        | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Headers - atx           | **Yes** | Yes     | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Headers - atx closed    | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Block Quotes            | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Unordered Lists         | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Ordered Lists           | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Task List Items         | No      | No      | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Indented Defn. Lists    | No      | No      | No      | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Verbatim (Indent Code)  | No      | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Fenced Code             | No      | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Horizontal Rule         | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Pipe Tables             | No      | No      | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Table Captions          | No      | No      | No      | **Yes** | No      | **Yes** | No       |
|            | Figure Image            | No      | No      | No      | **Yes** | No      | **Yes** | No       |
|            | Image (Block) Captions  | No      | No      | No      | **Yes** | No      | **Yes** | No       |
|            | Table of Contents       | No      | No      | No      | **Yes** | No      | No      | No       |
| **Span**   | Emphasis (Italic)       | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Strong (bold)           | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Strikethrough           | No      | No      | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Superscript Text        | No      | No      | No      | **Yes** | No      | **Yes** | No       |
|            | Subscript Text          | No      | No      | No      | **Yes** | No      | **Yes** | No       |
|            | Code                    | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Code w/escaped "`"      | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | [CriticMarkup]          | No      | No      | No      | **Yes** | No      | No      | No       |
|            | "--" en dash            | No      | No      | No      | **Yes** | No      | No      | **Yes**  |
|            | "---" em dash           | No      | No      | No      | **Yes** | No      | No      | **Yes**  |
|            | "..." ellipsis          | No      | No      | No      | **Yes** | No      | No      | **Yes**  |
|            | "<<>>" guillement       | No      | No      | No      | No      | No      | No      | **Yes**  |
|            | Emoji                   | No      | No      | ???     | No      | No      | **Yes** | No       |
|            | Dollar Math             | No      | No      | No      | **Yes** | No      | **Yes** | **Yes**  |
|            | Backslash Math          | No      | No      | No      | **Yes** | No      | No      | No       |
|            | Images                  | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Image Height/Width      | No      | No      | No      | **Yes** | No      | No      | No       |
|            | Automatic Links         | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Backslash Escapes       | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Links                   | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Link Titles             | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Ref-Style Links         | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Footnotes               | No      | No      | No      | **Yes** | **Yes** | **Yes** | **Yes**  |
|            | Abbreviations           | No      | No      | No      | **Yes** | **Yes** | No      | **Yes**  |
|            | Glossary                | No      | No      | No      | **Yes** | No      | No      | No       |
|            | Citations               | No      | No      | No      | **Yes** | No      | **Yes** | No       |
| **Other**  | Metadata                | No      | No      | **Yes** | **Yes** | No      | **Yes** | No       |
|            | Transclusion            | No      | No      | No      | **Yes** | No      | No      | No       |
|            | Variables               | No      | No      | No      | **Yes** | No      | No      | No       |
|            | Seq. numbered examples  | No      | No      | No      | No      | No      | **Yes** | No       |
        





[CriticMarkup]: http://criticmarkup.com
