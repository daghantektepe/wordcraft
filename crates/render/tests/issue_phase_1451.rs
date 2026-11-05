use wordcraft_doc::Document;
use wordcraft_layout::{LayoutCache, LayoutOptions, layout};
use wordcraft_render::{RenderOptions, render_page};

#[test]
fn issue_phase_1451_render_page_rgba_buffer_integrity() {
    let doc = Document::from_text("Render verification for issue_phase_1451");
    let mut cache = LayoutCache::new();
    let dl = layout(&doc, &mut cache, &LayoutOptions::default());
    assert!(!dl.pages.is_empty());

    let rendered = render_page(&doc, &dl.pages[0], 0.25, &RenderOptions::default());
    assert!(rendered.width > 0 && rendered.height > 0);
    let straight = rendered.to_straight();
    assert_eq!(straight.len(), (rendered.width as usize) * (rendered.height as usize) * 4);
}
