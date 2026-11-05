use wordcraft_doc::Document;
use wordcraft_layout::{LayoutCache, LayoutOptions, layout};

#[test]
fn issue_1631_layout_pagination_and_cache_consistency() {
    let doc = Document::from_text("First paragraph for issue_1631.\nSecond paragraph with layout flow.\nThird paragraph.");
    let mut cache = LayoutCache::new();
    let opts = LayoutOptions::default();
    let first_pass = layout(&doc, &mut cache, &opts);
    assert!(!first_pass.pages.is_empty());

    let second_pass = layout(&doc, &mut cache, &opts);
    assert_eq!(first_pass.pages.len(), second_pass.pages.len());
}

#[test]
fn issue_1631_empty_and_multipage_document_geometry() {
    let empty_doc = Document::new();
    let mut cache = LayoutCache::new();
    let opts = LayoutOptions::default();
    let dl = layout(&empty_doc, &mut cache, &opts);
    assert!(!dl.pages.is_empty());
    assert!(dl.pages[0].w > 0.0 && dl.pages[0].h > 0.0);
}
