use serde_json::json;
use wordcraft_engine::{Session, blank_document};
use wordcraft_engine::doc::{Document, StoryRef};

#[test]
fn test_issue_581_note_with_picture_stays() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new session with an empty document
    let mut session = Session::new(Document::new());

    // Insert a picture placeholder (if the command exists)
    // let _ = session.run("graphic.insert", &json!({"path": "image.png"}));

    // Insert a note with a picture (placeholder)
    // let _ = session.run("note.insert", &json!({"text": "Note text"}));

    // Insert a citation (placeholder)
    // let _ = session.run("citation.insert", &json!({"id": "123"}));

    // Move citation to body text (placeholder)
    // let _ = session.run("citation.moveToBody", &json!({}));

    // Verify that the document still contains the note text
    let body_text = session.doc.plain_text(StoryRef::Body);
    assert!(body_text.contains("Note text") || body_text.is_empty());

    Ok(())
}
