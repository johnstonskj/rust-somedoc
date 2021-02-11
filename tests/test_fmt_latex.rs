use somedoc::model::Document;
use somedoc::write::OutputFormat;

pub mod common;

const COMMON_PREAMBLE: &str = r###"\documentclass[twoside, 12pt, lettersize]{article}

\usepackage{amsmath}
\usepackage{csquotes}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{listings}
\newcommand{\thematicbreak}{\par\bigskip\noindent\hrulefill\par\bigskip}

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

  \subsection{Labels}

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

  \subsection{Other Properties}

  \thematicbreak

  Jump to: \hyperref[Concepts Hierarchy]{Concepts Hierarchy}\ref{Concepts Hierarchy} | \hyperref[Concepts]{Concepts}\ref{Concepts} | \hyperref[Collections]{Collections}\ref{Collections} | \hyperref[Appendix - RDF]{Appendix - RDF}\ref{Appendix - RDF}

  \thematicbreak

  \subsection{Concept Hierarchy}

  \begin{itemize}
    \item \textbf{First item}
    \item Second item
    \begin{itemize}
      \item \textit{Third item}
    \end{itemize}
    \item First item
  \end{itemize}

  \subsection{Appendix - RDF}

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
