use serde_json::json;
use wordcraft_engine::Session;
use wordcraft_engine::doc::{Document, props::VMerge};

#[test]
fn issue_5535_table_sort_fails_gracefully_on_vertically_merged_cells() {
    let mut s = Session::new(Document::new());
    s.run("insert.table", &json!({ "rows": 3, "cols": 1 })).unwrap();
    
    // Find the inserted table's path
    let (tp, _, _) = s.sel.focus.path.cell().expect("selection must be in table cell");
    if let Ok(t) = s.doc.table_mut(s.sel.focus.story, &tp) {
        if t.rows.len() >= 3 && !t.rows[1].cells.is_empty() && !t.rows[2].cells.is_empty() {
            t.rows[1].cells[0].props.vmerge = VMerge::Restart;
            t.rows[2].cells[0].props.vmerge = VMerge::Continue;
        }
    }
    
    // table.sort must refuse to sort rows with vertically merged cells
    let res = s.run("table.sort", &json!({ "column": 0, "descending": false }));
    assert!(res.is_err(), "table.sort must fail when table contains vertically merged cells");
    let err_msg = format!("{:?}", res.unwrap_err());
    assert!(
        err_msg.contains("cannot sort a table with vertically merged cells"),
        "error message should explain vertical merge restriction, got: {err_msg}"
    );
}

#[test]
fn issue_5740_spell_checker_state_preserved_across_documents() {
    let mut s = Session::new(Document::new());
    // Disable spell checking in prefs
    s.prefs.check_spelling = false;
    s.view.proofing = false;

    // Switch/new document
    let _ = s.run("file.new", &json!({}));
    assert!(!s.view.proofing, "new document view.proofing must honor prefs.check_spelling = false");
}
