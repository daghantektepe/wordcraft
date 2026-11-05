use wordcraft_doc::{Document, StoryRef};

#[test]
fn issue_2097_docx_roundtrip_and_integrity() {
    let mut doc = Document::from_text("WordCraft regression verification for issue_2097\nSecond paragraph content.");
    doc.core.title = "issue_2097".to_string();
    let bytes = wordcraft_docx::write(&doc).expect("write docx package");
    assert!(!bytes.is_empty());

    let loaded = wordcraft_docx::read(&bytes).expect("read docx package");
    let text = loaded.plain_text(StoryRef::Body);
    assert!(text.contains("WordCraft regression verification"));
    assert!(text.contains("Second paragraph content."));
    assert_eq!(loaded.core.title, "issue_2097");
}

#[test]
fn issue_2097_flat_xml_and_malformed_input_safety() {
    let doc = Document::from_text("Flat XML test for issue_2097");
    let flat_bytes = wordcraft_docx::write_flat(&doc).expect("write flat xml");
    assert!(!flat_bytes.is_empty());
    let parsed = wordcraft_docx::read_xml(&flat_bytes).expect("read flat xml");
    assert!(parsed.plain_text(StoryRef::Body).contains("Flat XML test"));

    assert!(wordcraft_docx::read(&[]).is_err());
    assert!(wordcraft_docx::read(b"not-a-valid-zip-archive").is_err());
}
