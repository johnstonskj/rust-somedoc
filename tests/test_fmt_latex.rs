use somedoc::model::Document;
use somedoc::write::OutputFormat;

pub mod common;

const COMMON_PREAMBLE: &str = r###"\documentclass[twoside, 12pt, lettersize]{article}

\usepackage{amsmath}
\usepackage{caption}
\usepackage{csquotes}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{listings}
\usepackage{ulem}
\newcommand{\thematicbreak}{\par\bigskip\noindent\hrulefill\par\bigskip}
\DeclareCaptionType{equfloat}[Equation][List of equations]

"###;

#[inline]
fn assert_latex_eq(part_fn: impl Fn() -> Document, expected: &str, preamble: Option<&str>) {
    common::assert_serialized_eq(
        &part_fn(),
        OutputFormat::Latex,
        &format!("{}{}", preamble.unwrap_or(COMMON_PREAMBLE), expected),
    )
}

#[test]
fn test_skos() {
    assert_latex_eq(
        common::skos::document,
        r###"\title{Scheme: Clothing shapes, patterns, and details}
\author{Simon}

\begin{document}

  \maketitle

  \section{Scheme: Clothing shapes, patterns, and details}

  %% TODO:
  %% - more nested lists
  %% - tables

  \textit{Terms commonly used to describe fashion items. It includes terms for outline, fit, elements, detailing, and patterns.}

  \url{http://amazon.com/vocabulary/fashion-design#DesignScheme}

  \label{Labels}\subsection{Labels}

  \begin{displayquote}
    \textbf{skos:prefLabel}

    \textbf{skos:altLabel}

  \end{displayquote}

  \begin{table}[h!bt]
    \centering
    \begin{tabular}{| l | l |}
      \hline
      Label text & Language \\
      \hline\hline
      Clothing shapes, patterns, and details & \textbf{en} \\
      \hline
    \end{tabular}
    \caption{Other labels}
  \end{table}

  \label{Other_Properties}\subsection{Other Properties}

  \thematicbreak

  Jump to: \hyperref[Concepts_Hierarchy]{Concepts Hierarchy} | \hyperref[Concepts]{Concepts} | \hyperref[Collections]{Collections} | \hyperref[Appendix_-_RDF]{Appendix - RDF}

  \thematicbreak

  \label{Concept_Hierarchy}\subsection{Concept Hierarchy}

  \begin{itemize}
    \item \textbf{First item}
    \item Second item
    \begin{itemize}
      \item \textit{Third item}
    \end{itemize}
    \item First item
  \end{itemize}

  \label{Appendix_-_RDF}\subsection{Appendix - RDF}

  \begin{lstlisting}[language=turtle]
@prefix foo: <...>
foo:bar foo:baz 12.
  \end{lstlisting}

  \begin{verbatim}
@prefix foo: <...>
foo:bar foo:baz 12.
  \end{verbatim}

\end{document}"###,
        None,
    );
}

#[test]
fn test_empty_document() {
    assert_latex_eq(
        common::parts::empty_document,
        r###"\begin{document}

  \maketitle

\end{document}"###,
        None,
    );
}

#[test]
fn test_document_with_title() {
    assert_latex_eq(
        common::parts::document_with_title,
        r###"\title{Test Document}

\begin{document}

  \maketitle

\end{document}"###,
        None,
    );
}

#[test]
fn test_document_with_heading() {
    assert_latex_eq(
        common::parts::document_with_heading,
        r###"\begin{document}

  \maketitle

  \section{Test Document}

\end{document}"###,
        None,
    );
}

#[test]
fn test_document_with_labeled_heading() {
    assert_latex_eq(
        common::parts::document_with_labeled_heading,
        r###"\begin{document}

  \maketitle

  \label{Test_Document}\section{Test Document}

\end{document}"###,
        None,
    );
}

#[test]
fn test_document_with_headings() {
    assert_latex_eq(
        common::parts::document_with_headings,
        r###"\begin{document}

  \maketitle

  \section{Section}

  \subsection{Sub-section}

  \subsubsection{Sub-sub-section}

  \subsubsubsection{Sub-sub-sub-section}

  \subsubsubsubsection{Sub-sub-sub-sub-section}

  \subsubsubsubsubsection{Sub-sub-sub-sub-sub-section}

  \subsubsubsubsubsubsection{Sub-sub-sub-sub-sub-sub-section}

\end{document}"###,
        None,
    );
}

#[test]
fn test_paragraph_alignment() {
    assert_latex_eq(
        common::parts::paragraph_alignment,
        r###"\begin{document}

  \maketitle

  left-aligned

  right-aligned

  center-aligned

  both-aligned

\end{document}"###,
        None,
    );
}

#[test]
fn test_document_with_front_matter() {
    assert_latex_eq(
        common::parts::document_with_front_matter,
        r###"\begin{document}

  \maketitle

  \tableofcontents

  \listoffigures

  \listoftables

  \listofequfloats

  \lstlistoflistings

  \section{Section One}

  \section{Section Two}

\end{document}"###,
        None,
    );
}

#[test]
fn test_unordered_list() {
    assert_latex_eq(
        common::parts::unordered_list,
        r###"\begin{document}

  \maketitle

  \begin{itemize}
    \item one
    \item two
    \item three
  \end{itemize}

\end{document}"###,
        None,
    );
}

#[test]
fn test_ordered_list() {
    assert_latex_eq(
        common::parts::ordered_list,
        r###"\begin{document}

  \maketitle

  \begin{enumerate}
    \item one
    \item two
    \item three
  \end{enumerate}

\end{document}"###,
        None,
    );
}

#[test]
fn test_labeled_ordered_list() {
    assert_latex_eq(
        common::parts::labeled_ordered_list,
        r###"\begin{document}

  \maketitle

  \label{lst1}\begin{enumerate}
    \item\label{lst1-itm1} one
    \item\label{lst1-itm2} two
    \item\label{lst1-itm3} three
  \end{enumerate}

\end{document}"###,
        None,
    );
}

#[test]
fn test_nested_unordered_list() {
    assert_latex_eq(
        common::parts::nested_unordered_list,
        r###"\begin{document}

  \maketitle

  \begin{itemize}
    \item one
    \item two
    \begin{itemize}
      \item inner one
      \item inner two
    \end{itemize}
    \item three
  \end{itemize}

\end{document}"###,
        None,
    );
}

#[test]
fn test_nested_ordered_list() {
    assert_latex_eq(
        common::parts::nested_ordered_list,
        r###"\begin{document}

  \maketitle

  \begin{enumerate}
    \item one
    \item two
    \begin{enumerate}
      \item inner one
      \item inner two
    \end{enumerate}
    \item three
  \end{enumerate}

\end{document}"###,
        None,
    );
}

#[test]
fn test_nested_mixed_lists() {
    assert_latex_eq(
        common::parts::nested_mixed_lists,
        r###"\begin{document}

  \maketitle

  \begin{itemize}
    \item one
    \item two
    \begin{enumerate}
      \item inner one
      \begin{itemize}
        \item inner inner one
      \end{itemize}
      \item inner two
    \end{enumerate}
    \item three
  \end{itemize}

\end{document}"###,
        None,
    );
}

#[test]
fn test_definition_list() {
    assert_latex_eq(
        common::parts::definition_list,
        r###"\begin{document}

  \maketitle

  \begin{description}
    \item [Universe] Big, really big
  \end{description}

\end{document}"###,
        None,
    );
}

#[test]
fn test_image_block() {
    assert_latex_eq(
        common::parts::image_block,
        r###"\begin{document}

  \maketitle

  \begin{figure}[h!bt]
    \centering
    \includegraphics{https://example.org/example.png}
  \end{figure}

\end{document}"###,
        None,
    );
}

#[test]
fn test_image_block_with_label_and_caption() {
    assert_latex_eq(
        common::parts::image_block_with_label_and_caption,
        r###"\begin{document}

  \maketitle

  \begin{figure}[h!bt]
    \centering
    \includegraphics{https://example.org/example.png}
    \caption{An Example Image}
    \label{img:example}
  \end{figure}

\end{document}"###,
        None,
    );
}

#[test]
fn test_math_block() {
    assert_latex_eq(
        common::parts::math_block,
        r###"\begin{document}

  \maketitle

  \begin{equfloat}[h!bt]
    \begin{equation}
      x=2+2^2
    \end{equation}
  \end{equfloat}

\end{document}"###,
        None,
    );
}

#[test]
fn test_math_block_with_label_and_caption() {
    assert_latex_eq(
        common::parts::math_block_with_label_and_caption,
        r###"\begin{document}

  \maketitle

  \begin{equfloat}[h!bt]
    \begin{equation}
      x=2+2^2
    \end{equation}
    \caption{Example Math}
    \label{math:example}
  \end{equfloat}

\end{document}"###,
        None,
    );
}

#[test]
fn test_block_quote() {
    assert_latex_eq(
        common::parts::block_quote,
        r###"\begin{document}

  \maketitle

  \begin{displayquote}
    a block quote

  \end{displayquote}

\end{document}"###,
        None,
    );
}

#[test]
fn test_nested_block_quotes() {
    assert_latex_eq(
        common::parts::nested_block_quotes,
        r###"\begin{document}

  \maketitle

  \begin{displayquote}
    a block quote

    \begin{displayquote}
      another block quote

    \end{displayquote}

  \end{displayquote}

\end{document}"###,
        None,
    );
}

#[test]
fn test_text_styles() {
    assert_latex_eq(
        common::parts::text_styles,
        r###"\begin{document}

  \maketitle

  Here is some plain \textbf{bold} \textit{italic} \texttt{mono} \texttt{code} plain \sout{strikethrough} \underline{underline} \textsc{small caps} \textsuperscript{superscript} \textsubscript{subscript} text.

\end{document}"###,
        None,
    );
}

#[test]
fn test_nested_text_styles() {
    assert_latex_eq(
        common::parts::nested_text_styles,
        r###"\begin{document}

  \maketitle

  Here is some \textbf{\textit{bold italic}} text.

  Here is some bold italic plain text.

  Here is some \textit{bold plain italic} text.

\end{document}"###,
        None,
    );
}
