use crate::resume::*;
use glyph_brush_layout::ab_glyph::Font;
use glyph_brush_layout::GlyphPositioner;
use itertools::Itertools;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use std::io::Read;
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

fn mm_to_px(mm: f32) -> f32 {
    mm * 96.0 / 25.4
}

fn px_to_mm(px: f32) -> f32 {
    px * 25.4 / 96.0
}

pub fn create_pdf_from_resume(resume: &Resume) {
    // Create a pdf letter size
    let (doc, page1, layer1) = PdfDocument::new("test", Mm(215.9), Mm(279.4), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Fonts
    // Load in the font data
    let normal_font = {
        let mut font_file = File::open("./fonts/times_new_roman.ttf").unwrap();
        let mut font_data = Vec::with_capacity(font_file.metadata().unwrap().len() as usize);
        font_file.read_to_end(&mut font_data).unwrap();
        font_data
    };

    let bold_font = {
        let mut font_file = File::open("./fonts/times_new_roman_bold.ttf").unwrap();
        let mut font_data = Vec::with_capacity(font_file.metadata().unwrap().len() as usize);
        font_file.read_to_end(&mut font_data).unwrap();
        font_data
    };

    // Load the font reference for glyph brush layout
    let gbl_normal_font = glyph_brush_layout::ab_glyph::FontRef::try_from_slice(&normal_font).unwrap();
    let gbl_bold_font = glyph_brush_layout::ab_glyph::FontRef::try_from_slice(&bold_font).unwrap();
    let gbl_fonts = &[gbl_normal_font, gbl_bold_font];

    let normal_writing_font = doc.add_external_font(normal_font.as_slice()).unwrap();
    let bold_writing_font = doc.add_external_font(bold_font.as_slice()).unwrap();

    let mut current_line_height = 269.4;

    let mut write_to_page = |text_to_write: &str, left_margin: f32, width: f32, font_size: f32, font_id: usize| {
        // calculate the glyph positions using glyph_brush_layout
        let glyphs = glyph_brush_layout::Layout::default().calculate_glyphs(
            gbl_fonts,
            &glyph_brush_layout::SectionGeometry {
                bounds: (mm_to_px(width), f32::INFINITY),
                ..Default::default()
            },
            &[glyph_brush_layout::SectionText {
                text: text_to_write,
                scale: gbl_fonts[font_id].pt_to_px_scale(font_size).unwrap(),
                font_id: glyph_brush_layout::FontId(font_id),
            }],
        );

        // make sure the number of glyphs matches the number of chars in the sample text
        assert_eq!(glyphs.len(), text_to_write.chars().count());

        // group the glyphs by y position
        let line_starts = glyphs
            .iter()
            .enumerate()
            .chunk_by(|(_, glyph)| glyph.glyph.position.y)
            .into_iter()
            .map(|(y, mut group)| (y, group.next().unwrap().0))
            .collect::<Vec<_>>();

        // get the minimum y position
        let min = glyphs
            .iter()
            .map(|glyph| glyph.glyph.position.y)
            .fold(f32::INFINITY, |a, b| a.min(b));

        // need a peekable iterator so we can see where the next line starts
        let mut iter = line_starts.iter().peekable();

        let mut line_diff = 0.0;

        // iterate over the line_starts and draw the text
        loop {
            // get the next line start, if there is none then we break out of the loop
            let Some((y, start)) = iter.next() else {
                break;
            };

            // peek into the line start after that to get the end index,
            // if there is none (we're at the last line of the loop) then we use the length of the sample text
            let end = if let Some((_, end)) = iter.peek() {
                *end
            } else {
                text_to_write.chars().count()
            };

            // Slice up the text
            // if you know you're only dealing with ASCII characters you can simplify this as
            // `let line = &sample[*start..end];`
            // which saves on an allocation to a String;
            // or you can use char_indices to get the byte indices and slice that way
            let line = text_to_write
                .chars()
                .skip(*start)
                .take(end - start)
                .collect::<String>();

            let font = match font_id {
                1 => {
                    &bold_writing_font
                } 
                _ => {
                    &normal_writing_font
                }            
            };

            current_layer.use_text(
                line.trim(),
                font_size,
                Mm(left_margin),
                Mm(current_line_height + px_to_mm(min) - px_to_mm(*y)),
                font,
            );
            // This needs to be changed because it is very inefficient
            line_diff = px_to_mm(*y);
        }
        current_line_height -= line_diff;
    };

    // Add Skills
    write_to_page("Skills", 20.0, 160.0, 25.0, 1);

    for category in resume.get_skills().iter() {
        write_to_page(&capitalize(category.0.clone()), 20.0, 160.0, 25.0, 1);

        let mut skills = String::new();
        for item in category.1.iter() {
            skills.push_str(&std::format!("{} ", item.sortable));
        }
        write_to_page(&skills, 20.0, 160.0, 14.0, 0);
    }


    /*
    // Add Work Experience
    // current_layer.set_font(&font, FontStyle::SectionHeader.get_font_size()); // Section Header
    // current_layer.set_fill_color(FontStyle::SectionHeader.get_font_color());
    // current_layer.write_text("Work Experience", &font);
    write_section_header(&current_layer, "Work Experience");

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
    // current_layer.set_font(&font, FontStyle::SectionHeader.get_font_size()); // Section Header
    // current_layer.set_fill_color(FontStyle::SectionHeader.get_font_color());
    // current_layer.write_text("Education", &font);
    write_section_header(&current_layer, "Education");
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
    */

    // current_layer.end_text_section();

    // Save the pdf in the designated directory
    doc.save(&mut BufWriter::new(File::create("./res/test.pdf").unwrap()))
        .unwrap();
}
