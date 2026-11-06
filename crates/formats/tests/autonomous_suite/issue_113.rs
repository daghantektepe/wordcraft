use wordcraft_doc::{Document, SectionProps, StoryRef};
use wordcraft_formats::ExportOptions;

#[test]
fn issue_113_multi_format_roundtrip_verification() {
    let doc = Document::from_text("Format verification for issue_113\nLine two with UTF-8: café αβγ.");
    for ext in ["txt", "md", "html", "rtf", "odt", "fodt", "tex"] {
        let exported = wordcraft_formats::export(ext, &doc)
            .expect("supported format")
            .expect("export succeeded");
        assert!(!exported.is_empty(), "export empty for {ext}");

        let imported = wordcraft_formats::import(ext, &exported)
            .expect("supported import format")
            .expect("import succeeded");
        let body = imported.plain_text(StoryRef::Body);
        assert!(body.contains("Format verification"), "roundtrip failed for {ext}");
    }
}

#[test]
fn issue_113_export_with_options_and_page_import() {
    let doc = Document::from_text("Page setup and options test for issue_113");
    let opts = ExportOptions { comments: true };
    let html_bytes = wordcraft_formats::export_with("html", &doc, &opts)
        .expect("supported")
        .expect("html export with comments");
    let page = SectionProps::default();
    let loaded = wordcraft_formats::import_on("html", &html_bytes, &page)
        .expect("supported")
        .expect("import on page");
    assert!(loaded.plain_text(StoryRef::Body).contains("Page setup and options test"));
}
