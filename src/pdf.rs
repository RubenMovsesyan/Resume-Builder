use crate::resume::*;
use printpdf::*;
use latex::{DocumentClass, Element, Document, Section, Align};
use genpdf::*;
use std::fs::File;
use std::io::BufWriter;

use std::io::Write;

enum FontStyle {
    SectionHeader,
    ItemHeader,
    ExtraInfo,
    Normal,
}

const GRAY: printpdf::Color = Color::Rgb({
    let r = 100.0 / 256.0;
    let g = 100.0 / 256.0;
    let b = 100.0 / 256.0;
    Rgb {
        r,
        g,
        b,
        icc_profile: None,
    }
});

const BLACK: printpdf::Color = Color::Rgb({
    let r = 0.0;
    let g = 0.0;
    let b = 0.0;
    Rgb {
        r,
        g,
        b,
        icc_profile: None,
    }
});

impl FontStyle {
    fn get_font_size(&self) -> f32 {
        match self {
            FontStyle::SectionHeader | FontStyle::ItemHeader => 25.0,
            FontStyle::ExtraInfo | FontStyle::Normal => 14.0,
        }
    }

    fn get_font_color(&self) -> Color {
        match self {
            FontStyle::SectionHeader => GRAY,
            _ => BLACK,
        }
    }
}

fn capitalize(string: String) -> String {
    let mut output = String::new();
    let mut first = true;

    for character in string.chars() {
        if first {
            output.push(character.to_ascii_uppercase());
            first = false;
        } else {
            output.push(character);
        }
    }

    output
}

// pub fn gen_latex_from_resume(resume: &Resume) {
//     let mut doc = Document::new(DocumentClass::Article);

//     doc.preamble.title("Test Document");
//     doc.preamble.author("Me");

//     doc.push(Element::TitlePage)
//     .push(Element::ClearPage)
//     .push(Element::TableOfContents)
//     .push(Element::ClearPage);

//     let mut section_1 = Section::new("Section 1");
//     section_1.push("This is the first Section")
//     .push("And the second part of the first section");

//     doc.push(section_1);

//     let rendered = latex::print(&doc).unwrap();

//     // println!("{}", rendered);

//     let mut f = File::create("./res/test.tex").unwrap();
//     let _ = write!(f, "{}", rendered);
// }

// pub fn generate_pdf_from_resume(resume: &Resume) {
//     let font_family = genpdf::fonts::from_files("./fonts", "times", None).expect("Failed to load font");
    

//     let mut doc = genpdf::Document::new(font_family);
// }

pub fn create_pdf_from_resume(resume: &Resume) {
    // Create a pdf letter size
    let (doc, page1, layer1) = PdfDocument::new("test", Mm(215.9), Mm(279.4), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Fonts
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    // FIXME fix repeat code

    current_layer.begin_text_section();

    current_layer.set_text_cursor(Mm(10.0), Mm(269.4));

    // Add Skills
    current_layer.set_font(&font, FontStyle::SectionHeader.get_font_size()); // Section Header
    current_layer.set_fill_color(FontStyle::SectionHeader.get_font_color());
    current_layer.write_text("Skills", &font);
    current_layer.set_line_height(25.0);
    current_layer.add_line_break();
    for category in resume.get_skills().iter() {
        current_layer.set_font(&font_bold, FontStyle::ItemHeader.get_font_size());
        current_layer.set_fill_color(FontStyle::ItemHeader.get_font_color());
        current_layer.write_text(capitalize(category.0.clone()), &font_bold);
        current_layer.set_line_height(14.0);
        current_layer.add_line_break();

        current_layer.set_font(&font, FontStyle::Normal.get_font_size());
        current_layer.set_fill_color(FontStyle::Normal.get_font_color());
        let mut skills = String::new();
        for item in category.1.iter() {
            skills.push_str(&std::format!("{} ", item.sortable));
        }
        current_layer.write_text(skills.clone(), &font);
        current_layer.set_line_height(25.0);
        current_layer.add_line_break();
    }

    // Add Work Experience
    current_layer.set_font(&font, FontStyle::SectionHeader.get_font_size()); // Section Header
    current_layer.set_fill_color(FontStyle::SectionHeader.get_font_color());
    current_layer.write_text("Work Experience", &font);

    for experience in resume.get_work_experience().iter() {
        let e = &experience.sortable;
        // Job title and dates
        current_layer.set_line_height(FontStyle::ItemHeader.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font_bold, FontStyle::ItemHeader.get_font_size());
        current_layer.set_fill_color(FontStyle::ItemHeader.get_font_color());
        current_layer.write_text(
            format!("{} {}", e.get_job_title(), {
                let mut o = String::new();

                let start_date = e.get_job_start();
                let end_date = e.get_job_end();

                o.push_str(&format!("{}", start_date));
                match end_date {
                    Some(date) => o.push_str(&format!("-{}", date)),
                    None => {}
                }

                o
            }),
            &font_bold,
        );

        // Company and location
        current_layer.set_line_height(FontStyle::ExtraInfo.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font, FontStyle::ExtraInfo.get_font_size());
        current_layer.set_fill_color(FontStyle::ExtraInfo.get_font_color());
        current_layer.write_text(
            format!("{} {}", e.get_company_name(), {
                let mut o = String::new();
                match e.get_job_location() {
                    Some(location) => o = format!("{}", location),
                    None => {}
                }
                o
            }),
            &font,
        );

        // Job descriptions
        // How is this the easiet one to implement -_-
        current_layer.set_line_height(FontStyle::Normal.get_font_size());
        current_layer.set_font(&font, FontStyle::Normal.get_font_size());
        current_layer.set_fill_color(FontStyle::Normal.get_font_color());
        for desc in e.get_job_description() {
            current_layer.add_line_break();
            current_layer.write_text(format!("* {}", desc.clone()), &font);
        }
    }

    // Add Education
    current_layer.set_line_height(FontStyle::SectionHeader.get_font_size());
    current_layer.add_line_break();
    current_layer.set_font(&font, FontStyle::SectionHeader.get_font_size()); // Section Header
    current_layer.set_fill_color(FontStyle::SectionHeader.get_font_color());
    current_layer.write_text("Education", &font);
    for education in resume.get_education() {
        let e = &education.sortable;

        // School and dates
        current_layer.set_line_height(FontStyle::ItemHeader.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font_bold, FontStyle::ItemHeader.get_font_size());
        current_layer.set_fill_color(FontStyle::ItemHeader.get_font_color());
        current_layer.write_text(
            format!("{} {}", e.get_school_name(), {
                let mut o = String::new();

                let start_date = e.get_education_start();
                let end_date = e.get_education_end();

                o.push_str(&format!("{}", start_date));
                match end_date {
                    Some(date) => o.push_str(&format!("-{}", date)),
                    None => {}
                }

                o
            }),
            &font_bold,
        );

        // Major and location
        current_layer.set_line_height(FontStyle::ExtraInfo.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font, FontStyle::ExtraInfo.get_font_size());
        current_layer.set_fill_color(FontStyle::ExtraInfo.get_font_color());
        current_layer.write_text(
            format!(
                "{} {}",
                {
                    let majors = e.get_major();
                    let mut o = String::new();

                    for (index, major) in majors.into_iter().enumerate() {
                        if index != majors.len() {
                            o.push_str(&format!("{} and", major));
                        } else {
                            o.push_str(&format!("{}", major));
                        }
                    }

                    o
                },
                {
                    let mut o = String::new();

                    o.push_str(&format!("{}", e.get_education_start()));

                    match e.get_education_end() {
                        Some(date) => o.push_str(&format!("-{}", date)),
                        None => {}
                    }

                    o
                }
            ),
            &font,
        );

        // Minor(s)
        current_layer.set_line_height(FontStyle::ExtraInfo.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font, FontStyle::ExtraInfo.get_font_size());
        current_layer.set_fill_color(FontStyle::ExtraInfo.get_font_color());
        current_layer.write_text(
            format!("Minor in {}", {
                let minors = e.get_minor();
                let mut o = String::new();

                for (index, minor) in minors.into_iter().enumerate() {
                    if index != minors.len() {
                        o.push_str(&format!("{} and", minor));
                    } else {
                        o.push_str(&format!("{}", minor));
                    }
                }

                o
            }),
            &font,
        );

        // Relevant coursework
        current_layer.set_line_height(FontStyle::Normal.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font, FontStyle::Normal.get_font_size());
        current_layer.set_fill_color(FontStyle::Normal.get_font_color());
        current_layer.write_text(
            format!("* Relevant Coursework {}", {
                let mut o = String::new();
                let courses = e.get_coursework();

                for (index, course) in courses.into_iter().enumerate() {
                    if index != courses.len() {
                        o.push_str(&format!("{}, ", course));
                    } else {
                        o.push_str(&format!("{}", course));
                    }
                }

                o
            }),
            &font,
        );

        // GPA
        current_layer.set_line_height(FontStyle::Normal.get_font_size());
        current_layer.add_line_break();
        current_layer.set_font(&font, FontStyle::Normal.get_font_size());
        current_layer.set_fill_color(FontStyle::Normal.get_font_color());
        current_layer.write_text(format!("* {} GPA", e.get_gpa()), &font);
    }

    current_layer.end_text_section();

    // Save the pdf in the designated directory
    doc.save(&mut BufWriter::new(File::create("./res/test.pdf").unwrap()))
        .unwrap();
}
