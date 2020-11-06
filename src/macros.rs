/*!
One-line description.

More detailed description, with

# Example

*/

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! doc {
    ($statements:block) => {{
        let mut doc = Document::default();

        $statements

        doc
    }};
}

#[macro_export]
macro_rules! textbf {
    ($s:expr) => {
        Text::bold($s);
    };
}

#[macro_export]
macro_rules! textit {
    ($s:expr) => {
        Text::italic($s);
    };
}

#[macro_export]
macro_rules! textsl {
    ($s:expr) => {
        Text::slanted($s);
    };
}

#[macro_export]
macro_rules! texttt {
    ($s:expr) => {
        Text::mono($s);
    };
}

#[macro_export]
macro_rules! text {
    ($s:expr) => {
        Text::plain($s);
    };
}

#[macro_export]
macro_rules! textsc {
    ($s:expr) => {
        Text::small_caps($s);
    };
}
