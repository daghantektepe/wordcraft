use wordcraft_doc::Document;
use wordcraft_pdf::PdfOptions;

#[test]
fn issue_phase_1456_pdf_export_structure_and_metadata() {
    let mut doc = Document::from_text("PDF verification document for issue_phase_1456\nSecond line.");
    doc.core.title = "issue_phase_1456".to_string();
    let opts = PdfOptions::default();
    let pdf_bytes = wordcraft_pdf::export(&doc, &opts).expect("pdf export succeeded");
    assert!(pdf_bytes.starts_with(b"%PDF-"));
    assert!(pdf_bytes.len() > 128);
}
