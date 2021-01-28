use proptest::prelude::*;
use somedoc::model::inline::HyperLink;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_link_alt_text() {}

#[test]
fn test_link_alt_content() {}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*", l in "\\PC*") {
        let _ = HyperLink::external(&s);
        let _ = HyperLink::external_with_label_str(&s, &l);
    }
}
