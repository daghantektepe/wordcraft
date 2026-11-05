use serde_json::json;
use wordcraft_engine::Session;
use wordcraft_engine::doc::{Document, StoryRef};

#[test]
fn issue_4105_session_commands_and_equation_safety() {
    let mut s = Session::new(Document::new());
    s.run("text.insert", &json!({ "text": "WordCraft issue_4105 " })).unwrap();
    let _ = s.run("equation.insert", &json!({}));
    let _ = s.run("equation.type", &json!({ "text": "x+1" }));
    let _ = s.run("equation.insertBuiltUp", &json!({ "text": "\\alpha" }));
    let _ = s.run("equation.deleteSurrounding", &json!({ "before": 1, "after": 0 }));

    let body_txt = s.doc.plain_text(StoryRef::Body);
    assert!(body_txt.contains("WordCraft issue_4105"));
}

#[test]
fn issue_4105_undo_redo_and_formatting_operations() {
    let mut s = Session::new(Document::new());
    s.run("text.insert", &json!({ "text": "Before formatting" })).unwrap();
    s.run("select.text", &json!({ "text": "Before" })).unwrap();
    s.run("format.bold", &json!({})).unwrap();
    s.run("edit.undo", &json!({})).unwrap();
    assert!(s.can_redo());
    s.run("edit.redo", &json!({})).unwrap();
    assert!(s.doc.plain_text(StoryRef::Body).contains("Before formatting"));
}
