use docx_rust::*;
// use docx_rust::document::Paragraph;
// use docx_rust::formatting::Size;
// use docx_rust::Docx;
use crate::resume::*;
use document::{Numbering, Paragraph, ParagraphContent, Run, RunContent, Tab};
use formatting::{CharacterProperty, Fonts, Indent, IndentLevel, JustificationVal, NumberingProperty, ParagraphProperty};

// Helper capitalize funciton
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

pub fn generate_docx_from_resume(resume: &Resume) {
    let mut docx = Docx::default();

    // Text properties for each section
    let section_header = CharacterProperty::default()
        .color(0x333333)
        .size(42isize)
        .fonts(Fonts::default().ascii("Times New Roman"));

    let item_header = CharacterProperty::default()
        .color(0x000000)
        .size(42isize)
        .bold(true)
        .fonts(Fonts::default().ascii("Times New Roman"));

    let normal_text = CharacterProperty::default()
        .color(0x000000)
        .size(24isize)
        .fonts(Fonts::default().ascii("Times New Roman"));

    // Closures for easy writing to the docx file
    let write_section_header = |d: &mut Docx, text: &str| {
        d.document.push(
            Paragraph::default().push(
                Run::default()
                    .property(section_header.clone())
                    .push_text(text.to_string()),
            ),
        );
    };

    let write_item_header = |d: &mut Docx, text: &str| {
        d.document.push(
            Paragraph::default().push(
                Run::default()
                    .property(item_header.clone())
                    .push_text(text.to_string()),
            ),
        );
    };

    let write_normal = |d: &mut Docx, text: &str| {
        d.document.push(
            Paragraph::default().push(
                Run::default()
                    .property(normal_text.clone())
                    .push_text(text.to_string()),
            ),
        );
    };


    //TODO: Find a way to make unordered lists work to make document parsing much easier
    let write_bullets = |d: &mut Docx, text: &Vec<String>| {
        let mut indent = Indent::default();
        indent.left = Some(42isize);
        for item in text {
            d.document.push(
                Paragraph::default()
                    .push(
                        Run::default()
                            .push(RunContent::Tab(Tab::default()))
                            .push_text(format!("• {}", item))
                    )
            );
        }
    };

    // Skills heading
    write_section_header(&mut docx, "Skills");

    // Add skills
    for category in resume.get_skills().iter() {
        write_item_header(&mut docx, &capitalize(category.0.clone()));

        let mut skills = String::new();
        for item in category.1.iter() {
            skills.push_str(&std::format!("{} ", item.sortable));
        }

        write_normal(&mut docx, &skills);
    }

    // Add Work experience
    write_section_header(&mut docx, "Work Experience");

    for experience in resume.get_work_experience().iter() {
        let e = &experience.sortable;

        write_item_header(
            &mut docx,
            &format!("{} {}", e.get_job_title(), {
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
        );

        write_normal(
            &mut docx,
            &format!("{} {}", e.get_company_name(), {
                let mut o = String::new();
                match e.get_job_location() {
                    Some(location) => o = format!("{}", location),
                    None => {}
                }
                o
            }),
        );

        write_bullets(
            &mut docx,
            e.get_job_description()
        );
    }

    docx.write_file("./res/test.docx").unwrap();
}
