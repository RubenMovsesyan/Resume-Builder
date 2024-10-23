use printpdf::*;
use std::fs::File;
use std::io::BufWriter;



pub fn create_pdf() {
    let (doc, page1, layer1) = PdfDocument::new("test", Mm(215.9), Mm(279.4), "Layer 1");

    doc.save(&mut BufWriter::new(File::create("./res/test.pdf").unwrap())).unwrap();
}