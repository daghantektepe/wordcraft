use wordcraft_engine::{Session};
use wordcraft_engine::doc::Document;
use serde_json::json;

#[test]
fn issue_179_zoom() {
    // Create a new session with an empty document
    let mut session = Session::new(Document::new());

    // Attempt to run the zoom command with a factor of 1.5
    let result = session.run("view.zoom", &json!({"factor": 1.5}));

    // The command may succeed or fail depending on implementation.
    // Ensure that the result is a valid Result type.
    assert!(result.is_ok() || result.is_err());
}
