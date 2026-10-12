//! End-to-end Tests.
//! If tests are only on a specific crate, it should go inside this same crate.

use rtod::api::Detection;

#[test]
fn detection_is_reexported() {
    let _ = std::mem::size_of::<Detection>();
}
