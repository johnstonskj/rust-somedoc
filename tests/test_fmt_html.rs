use somedoc::model::Document;
use somedoc::write::OutputFormat;

pub mod common;

#[inline]
fn assert_html_eq(part_fn: impl Fn() -> Document, expected: &str) {
    common::assert_serialized_eq(&part_fn(), OutputFormat::Html, expected)
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
    <p><a href="http://amazon.com/vocabulary/fashion-design#DesignScheme"></p>
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
    <p>Jump to: <a href="#Concepts_Hierarchy"> | <a href="#Concepts"> | <a href="#Collections"> | <a href="#Appendix_-_RDF"></p>
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
    );
}

#[test]
fn test_empty_document() {
    assert_html_eq(
        common::parts::empty_document,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
  </body>
</html>"###,
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
    );
}

#[test]
fn test_document_with_heading() {
    assert_html_eq(
        common::parts::document_with_heading,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
    <h1>Test Document</h1>
  </body>
</html>"###,
    );
}

#[test]
fn test_document_with_headings() {
    assert_html_eq(
        common::parts::document_with_headings,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
    <h1>Section</h1>
    <h2>Sub-section</h2>
    <h3>Sub-sub-section</h3>
    <h4>Sub-sub-sub-section</h4>
    <h5>Sub-sub-sub-sub-section</h5>
    <h6>Sub-sub-sub-sub-sub-section</h6>
    <h7>Sub-sub-sub-sub-sub-sub-section</h7>
  </body>
</html>"###,
    );
}

#[test]
fn test_unordered_list() {
    assert_html_eq(
        common::parts::unordered_list,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
    <ul>
      <li>one</li>
      <li>two</li>
      <li>three</li>
    </ul>
  </body>
</html>"###,
    );
}

#[test]
fn test_ordered_list() {
    assert_html_eq(
        common::parts::ordered_list,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
    <ol>
      <li>one</li>
      <li>two</li>
      <li>three</li>
    </ol>
  </body>
</html>"###,
    );
}

#[test]
fn test_nested_unordered_list() {
    assert_html_eq(
        common::parts::nested_unordered_list,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
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
    );
}

#[test]
fn test_nested_ordered_list() {
    assert_html_eq(
        common::parts::nested_ordered_list,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
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
    );
}

#[test]
fn test_nested_mixed_lists() {
    assert_html_eq(
        common::parts::nested_mixed_lists,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
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
    );
}

#[test]
fn test_definition_list() {
    assert_html_eq(
        common::parts::definition_list,
        r###"<html>
  <head>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/styles/default.min.css"></link>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.5.0/highlight.min.js"></script>
  </head>
  <body>
    <dl>
      <dt>Universe</dt>
      <dd>Big, really big</dd>
    </dl>
  </body>
</html>"###,
    );
}
