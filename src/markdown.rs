use crate::resume::*;
use std::fs::File;
use std::fs;
// use std::io::prelude::*;



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


pub fn generate_markdown_from_resume(resume: &Resume) {
//    let mut file = File::create("./res/test.md").unwrap();
    let mut contents = String::new();
    
    
    let mut write_section_header = |c: &mut String, text: &str| {
        c.push_str(&format!("# {}\n", text.to_string()));
    };
    
    let mut write_item_header = |c: &mut String, text: &str| {
        c.push_str(&format!("## {}\n", text.to_string()));
    };
    

    let mut write_normal = |c: &mut String, text: &str| {
        c.push_str(&format!("{}\n", text.to_string()));
    };

    let mut write_bullets = |c: &mut String, text: &Vec<String>| {
        for item in text {
            c.push_str(&format!("* {}\n", item));
        }
    };

    // Skills Heading
    write_section_header(&mut contents, "Skills");
    
    // Add Skills
    for category in resume.get_skills().iter() {
        write_item_header(&mut contents, &capitalize(category.0.clone()));

        let mut skills = String::new();
        for item in category.1.iter() {
            skills.push_str(&format!("{} ", item.sortable));
        }

        write_normal(&mut contents, &skills);
    }
    

    // Add Work experience
    write_section_header(&mut contents, "Work Experience");

    for experience in resume.get_work_experience().iter() {
        let e = &experience.sortable;
        
        // Work experience title
        write_item_header(
           &mut contents,
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
        
        // Work experience company
        write_normal(
            &mut contents,
            &format!("{} {}", e.get_company_name(), {
                let mut o = String::new();
                match e.get_job_location() {
                    Some(location) => o = format!("{}", location),
                    None => {}
                }
                o
            })
        );
        
        // Job description
        write_bullets(
            &mut contents,
            e.get_job_description()
        );
    }

    
    // Add Education
    write_section_header(&mut contents, "Education");

    for education in resume.get_education() {
        let e = &education.sortable;

        // School and dates
        write_item_header(
            &mut contents,
            &format!("{} {}", e.get_school_name(), {
                let mut o = String::new();

                let start_date = e.get_education_start();
                let end_date = e.get_education_end();

                o.push_str(&format!("{}", start_date));
                match end_date {
                    Some(date) => o.push_str(&format!("- {}", date)),
                    None => {}
                }

                o
            })
        );

        // Major and location
        write_normal(
            &mut contents,
            &format!("{} {}", {
                let majors = e.get_major();
                let mut o = String::new();

                for (index, major) in majors.into_iter().enumerate() {
                    if index != majors.len() - 1 {
                        o.push_str(&format!("{} and", major));
                    } else {
                        o.push_str(&format!("{}", major));
                    }
                }

                o
            }, {
                let mut o = String::new();

                o.push_str(&format!("{}", e.get_education_start()));

                match e.get_education_end() {
                    Some(date) => o.push_str(&format!(" - {}", date)),
                    None => {}
                }

                o
            })
        );

        // Minor(s)
        write_normal(
            &mut contents, 
            &format!("Minor in {}", {
                let minors = e.get_minor();
                let mut o = String::new();

                
                for (index, minor) in minors.into_iter().enumerate() {
                    if index != minors.len() - 1 {
                        o.push_str(&format!("{} and ", minor));
                    } else {
                        o.push_str(&format!("{}", minor));
                    }
                }

                o
            })
        );

        // Relavant coursework
        write_normal(
            &mut contents,
            &format!("* Relevant Coursework: {}", {
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
            })
        );

        // GPA
        write_normal(
            &mut contents,
            &format!("* {} GPA", e.get_gpa())
        );


        // Projects
        write_section_header(&mut contents, "Projects");

        for project in resume.get_projects().iter() {
            let e = &project.sortable;

            // Project title and dates
            write_item_header(
                &mut contents,
                &format!("{} {}",
                    e.get_project_name(),
                    {
                        let mut o = String::new();

                        let start_date = e.get_project_start();
                        let end_date = e.get_project_end();

                        o.push_str(&format!("{}", start_date));
                        match end_date {
                            Some(date) => o.push_str(&format!(" - {}", date)),
                            None => {}
                        }

                        o
                    }
                )
            );

            // Project descriptions
            write_bullets(&mut contents,
                e.get_project_description()        
            );
        }
    }
   
    fs::write("./res/test.md" , contents).expect("Unable to write file");
}