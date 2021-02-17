use somedoc::model::Document;
use somedoc::write::OutputFormat;

pub mod common;

const COMMON_PREAMBLE: &str = r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  "###;

#[inline]
fn assert_html_eq(part_fn: impl Fn() -> Document, expected: &str, preamble_included: bool) {
    common::assert_serialized_eq(
        &part_fn(),
        OutputFormat::Html,
        &format!(
            "{}{}",
            if preamble_included {
                ""
            } else {
                COMMON_PREAMBLE
            },
            expected
        ),
    )
}

#[test]
fn test_skos() {
    assert_html_eq(
        common::skos::document,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
    <title>Scheme: Clothing shapes, patterns, and details</title>
    <meta name="author" content="Simon">
  </head>
  <body>
    <h1>Scheme: Clothing shapes, patterns, and details</h1>
<!-- TODO:
- more nested lists
- tables -->
    <p><em>Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns.</em></p>
    <p><a href="http://amazon.com/vocabulary/fashion-design#DesignScheme"/></p>
    <h2 id="Labels">Labels</h2>
    <blockquote>
      <p><strong>skos:prefLabel</strong></p>
      <p><strong>skos:altLabel</strong></p>
    </blockquote>
    <table>
      <caption>Other labels</caption>
      <thead>
        <tr><th>Label text</th><th>Language</th></tr>
      </thead>
      </tbody>
        <tr><td>Clothing shapes, patterns, and details</td><td><strong>en</strong></td></tr>
      </tbody>
    </table>
    <h2 id="Other_Properties">Other Properties</h2>
    <hr/>
    <p>Jump to: <a href="#Concepts_Hierarchy">Concepts Hierarchy</a> | <a href="#Concepts">Concepts</a> | <a href="#Collections">Collections</a> | <a href="#Appendix_-_RDF">Appendix - RDF</a></p>
    <hr/>
    <h2 id="Concept_Hierarchy">Concept Hierarchy</h2>
    <ul>
      <li><strong>First item</strong></li>
      <li>Second item</li>
      <li>
        <ul>
          <li><em>Third item</em></li>
        </ul>
      </li>
      <li>First item</li>
    </ul>
    <h2 id="Appendix_-_RDF">Appendix - RDF</h2>
    <pre>
      <code class="turtle">@prefix foo: <...>
foo:bar foo:baz 12.
      </code>
    </pre>
    <pre>@prefix foo: <...>
foo:bar foo:baz 12.
    </pre>
  </body>
</html>"###,
        true,
    );
}

#[test]
fn test_empty_document() {
    assert_html_eq(
        common::parts::empty_document,
        r###"<body>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_document_with_title() {
    assert_html_eq(
        common::parts::document_with_title,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
    <title>Test Document</title>
  </head>
  <body>
  </body>
</html>"###,
        true,
    );
}

#[test]
fn test_document_with_heading() {
    assert_html_eq(
        common::parts::document_with_heading,
        r###"<body>
    <h1>Test Document</h1>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_document_with_labeled_heading() {
    assert_html_eq(
        common::parts::document_with_labeled_heading,
        r###"<body>
    <h1 id="Test_Document">Test Document</h1>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_document_with_headings() {
    assert_html_eq(
        common::parts::document_with_headings,
        r###"<body>
    <h1>Section</h1>
    <h2>Sub-section</h2>
    <h3>Sub-sub-section</h3>
    <h4>Sub-sub-sub-section</h4>
    <h5>Sub-sub-sub-sub-section</h5>
    <h6>Sub-sub-sub-sub-sub-section</h6>
    <h7>Sub-sub-sub-sub-sub-sub-section</h7>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_paragraph_alignment() {
    assert_html_eq(
        common::parts::paragraph_alignment,
        r###"<body>
    <p>left-aligned</p>
    <p>right-aligned</p>
    <p>center-aligned</p>
    <p>both-aligned</p>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_document_with_front_matter() {
    assert_html_eq(
        common::parts::document_with_front_matter,
        r###"<body>
    <h1>Section One</h1>
    <h1>Section Two</h1>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_unordered_list() {
    assert_html_eq(
        common::parts::unordered_list,
        r###"<body>
    <ul>
      <li>one</li>
      <li>two</li>
      <li>three</li>
    </ul>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_ordered_list() {
    assert_html_eq(
        common::parts::ordered_list,
        r###"<body>
    <ol>
      <li>one</li>
      <li>two</li>
      <li>three</li>
    </ol>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_labeled_ordered_list() {
    assert_html_eq(
        common::parts::labeled_ordered_list,
        r###"<body>
    <ol id="lst1">
      <li id="lst1-itm1">one</li>
      <li id="lst1-itm2">two</li>
      <li id="lst1-itm3">three</li>
    </ol>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_nested_unordered_list() {
    assert_html_eq(
        common::parts::nested_unordered_list,
        r###"<body>
    <ul>
      <li>one</li>
      <li>two</li>
      <li>
        <ul>
          <li>inner one</li>
          <li>inner two</li>
        </ul>
      </li>
      <li>three</li>
    </ul>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_nested_ordered_list() {
    assert_html_eq(
        common::parts::nested_ordered_list,
        r###"<body>
    <ol>
      <li>one</li>
      <li>two</li>
      <li>
        <ol>
          <li>inner one</li>
          <li>inner two</li>
        </ol>
      </li>
      <li>three</li>
    </ol>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_nested_mixed_lists() {
    assert_html_eq(
        common::parts::nested_mixed_lists,
        r###"<body>
    <ul>
      <li>one</li>
      <li>two</li>
      <li>
        <ol>
          <li>inner one</li>
          <li>
            <ul>
              <li>inner inner one</li>
            </ul>
          </li>
          <li>inner two</li>
        </ol>
      </li>
      <li>three</li>
    </ul>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_definition_list() {
    assert_html_eq(
        common::parts::definition_list,
        r###"<body>
    <dl>
      <dt>Universe</dt>
      <dd>Big, really big</dd>
    </dl>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_image_block() {
    assert_html_eq(
        common::parts::image_block,
        r###"<body>
    <div><img src="https://example.org/example.png"/></div>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_image_block_with_label_and_caption() {
    assert_html_eq(
        common::parts::image_block_with_label_and_caption,
        r###"<body>
    <div id="img:example"><img src="https://example.org/example.png"/></div>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_math_block() {
    assert_html_eq(
        common::parts::math_block,
        r###"<body>
    <div>\[ x=2+2^2 \]</div>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_math_block_with_label_and_caption() {
    assert_html_eq(
        common::parts::math_block_with_label_and_caption,
        r###"<body>
    <div id="math:example">\[ x=2+2^2 \]</div>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_block_quote() {
    assert_html_eq(
        common::parts::block_quote,
        r###"<body>
    <blockquote>
      <p>a block quote</p>
    </blockquote>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_nested_block_quotes() {
    assert_html_eq(
        common::parts::nested_block_quotes,
        r###"<body>
    <blockquote>
      <p>a block quote</p>
      <blockquote>
        <p>another block quote</p>
      </blockquote>
    </blockquote>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_text_styles() {
    assert_html_eq(
        common::parts::text_styles,
        r###"<body>
    <p>Here is some&#32;plain&#32;<strong>bold</strong>&#32;<em>italic</em>&#32;<code>mono</code>&#32;<code>code</code>&#32;plain&#32;<del>strikethrough</del>&#32;<ins>underline</ins>&#32;small caps&#32;<sup>superscript</sup>&#32;<sub>subscript</sub> text.</p>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_nested_text_styles() {
    assert_html_eq(
        common::parts::nested_text_styles,
        r###"<body>
    <p>Here is some <strong><em>bold italic</em></strong> text.</p>
    <p>Here is some bold italic plain text.</p>
    <p>Here is some <em>bold plain italic</em> text.</p>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_hyper_links() {
    assert_html_eq(
        common::parts::hyper_links,
        r###"<body>
    <p><a href="https://example.org/"/></p>
    <p><a href="https://example.org/">example</a></p>
    <p><a href="#section-2"/></p>
    <p><a href="#section-2">example</a></p>
  </body>
</html>"###,
        false,
    );
}

#[test]
fn test_complex_paragraph() {
    assert_html_eq(
        common::parts::complex_paragraph,
        r###"<body>
    <p>This paragraph has <a href="https://example.org/">a link</a>, some math:&nbsp;\( x=2+2^2 \), a line break,<br/>
an image:&nbsp;<img src="https://example.org/favicon.png"/>&nbsp;&mdash;&nbsp; all together!</p>
  </body>
</html>"###,
        false,
    );
}
