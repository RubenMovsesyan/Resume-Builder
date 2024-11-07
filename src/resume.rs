use crate::word_cloud::WordCloud;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, fs, io::Read};

// Necessary structs for resume
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Date {
    year: i32,
    month: Option<u8>,
    day: Option<u8>,
}

impl Date {
    pub fn new(year: i32, month: Option<u8>, day: Option<u8>) -> Self {
        Self { year, month, day }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} /", self.year)?;
        if self.month.is_some() {
            write!(f, " {} /", self.month.unwrap())?;
        }
        if self.day.is_some() {
            write!(f, " {}", self.day.unwrap())?;
        }

        Ok(())
    }
}

pub struct PhoneNumber {
    country_code: String,
    area_code: Option<String>,
    phone_number: String
}


impl PhoneNumber {
    // Phone number must be formatted like "+1 (555) 2321 123 123
    // the country code must have a + in front and the area code must be in ()
    pub fn new(phone_number_string: String) -> Self {
        let split_phone_number = phone_number_string.split(" ").collect::<Vec<&str>>();
        let mut cc = String::new();
        let mut ac = None;
        let mut pn = String::new();

        // Split the phone number from the string to the different components
        for split in split_phone_number.iter() {
            if split.contains("+") {
                cc = split.to_string();
            } else if split.contains("(") && split.contains(")") {
                ac = Some(split.to_string());
            } else {
                pn.push_str(&split);
            }
        }

        Self {
            country_code: cc,
            area_code: ac,
            phone_number: pn,
        }
    }
}

impl Display for PhoneNumber {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.country_code)?;

       match &self.area_code {
           Some(code) => { write!(f, " {}", code)? },
           None => {}
       }

       writeln!(f, " {}", self.phone_number)?;
       Ok(())
   }
}

//----------------------------------------------
#[derive(Serialize, Deserialize)]
pub struct Skills {
    //                  Skill name
    //                     |    Category
    //                     V       V
    skill_tree: HashMap<String, Vec<String>>,
}

impl Skills {
    pub fn new() -> Self {
        Self {
            skill_tree: HashMap::new(),
        }
    }

    pub fn add_skill(&mut self, skill_name: String, category: Option<String>) {
        let mut category_str = String::from("default");
        if category.is_some() {
            category_str = category.unwrap().to_ascii_lowercase();
        }

        // Check if the skill is already in the tree and add it if it is not
        if self.skill_tree.contains_key(&category_str) {
            self.skill_tree
                .get_mut(&category_str)
                .unwrap()
                .push(skill_name);
        } else {
            self.skill_tree.insert(category_str, vec![skill_name]);
        }
    }
}

impl Display for Skills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Skills:")?;
        for (category, skills) in self.skill_tree.iter() {
            writeln!(f, "    Category: {}", category)?;

            write!(f, "        ")?;
            for skill in skills.iter() {
                write!(f, "{skill}, ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

//__________________________________________

#[derive(Serialize, Deserialize)]
pub struct WorkExperience {
    job_title: String,
    company_name: String,
    job_location: Option<String>,
    job_description: Vec<String>,

    job_start: Date,
    job_end: Option<Date>,
}

impl WorkExperience {
    pub fn new(
        job_title: String,
        company_name: String,
        job_location: Option<String>,
        job_description: Vec<String>,
        job_start: Date,
        job_end: Option<Date>,
    ) -> Self {
        Self {
            job_title,
            company_name,
            job_location,
            job_description,
            job_start,
            job_end,
        }
    }

    pub fn from(work_experience: &WorkExperience) -> Self {
        Self {
            job_title: String::from(&work_experience.job_title),
            company_name: String::from(&work_experience.company_name),
            job_location: work_experience.job_location.clone(),
            job_description: work_experience.job_description.clone(),
            job_start: work_experience.job_start.clone(),
            job_end: work_experience.job_end.clone(),
        }
    }

    // getters
    pub fn get_job_title(&self) -> &String {
        &self.job_title
    }

    pub fn get_company_name(&self) -> &String {
        &self.company_name
    }

    pub fn get_job_location(&self) -> &Option<String> {
        &self.job_location
    }

    pub fn get_job_description(&self) -> &Vec<String> {
        &self.job_description
    }

    pub fn get_job_start(&self) -> &Date {
        &self.job_start
    }

    pub fn get_job_end(&self) -> &Option<Date> {
        &self.job_end
    }
}

impl Display for WorkExperience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    Job Title: {}", self.job_title)?;
        writeln!(f, "        Company Name: {}", self.company_name)?;

        if self.job_location.is_some() {
            writeln!(
                f,
                "        Location: {}",
                &self.job_location.clone().unwrap()
            )?;
        } else {
            writeln!(f, "        Location: N/A")?;
        }

        write!(f, "        Job Start: {}", self.job_start)?;

        if self.job_end.is_some() {
            write!(f, "        Job End: {}", &self.job_end.clone().unwrap())?;
        } else {
            writeln!(f, "        Job End: N/A")?;
        }

        writeln!(f, "        Job Description:")?;
        for item in self.job_description.iter() {
            writeln!(f, "            * {}", item)?;
        }

        Ok(())
    }
}

//__________________________________________

#[derive(Serialize, Deserialize)]
pub struct Education {
    school_name: String,
    major: Vec<String>,
    location: String,
    minor: Vec<String>,
    coursework: Vec<String>,
    gpa: f32,

    education_start: Date,
    education_end: Option<Date>,
}

impl Education {
    fn new(
        school_name: String,
        major: Vec<String>,
        location: String,
        minor: Vec<String>,
        coursework: Vec<String>,
        gpa: f32,
        education_start: Date,
        education_end: Option<Date>,
    ) -> Self {
        Self {
            school_name,
            major,
            location,
            minor,
            coursework,
            gpa,
            education_start,
            education_end,
        }
    }

    fn from(education: &Education) -> Self {
        Self {
            school_name: education.school_name.clone(),
            major: education.major.clone(),
            location: education.location.clone(),
            minor: education.minor.clone(),
            coursework: education.coursework.clone(),
            gpa: education.gpa,
            education_start: education.education_start.clone(),
            education_end: education.education_end.clone(),
        }
    }

    // getters
    pub fn get_school_name(&self) -> &String {
        &self.school_name
    }

    pub fn get_major(&self) -> &Vec<String> {
        &self.major
    }

    pub fn get_minor(&self) -> &Vec<String> {
        &self.minor
    }

    pub fn get_location(&self) -> &String {
        &self.location
    }

    pub fn get_coursework(&self) -> &Vec<String> {
        &self.coursework
    }

    pub fn get_gpa(&self) -> f32 {
        self.gpa
    }

    pub fn get_education_start(&self) -> &Date {
        &self.education_start
    }

    pub fn get_education_end(&self) -> &Option<Date> {
        &self.education_end
    }
}

impl Display for Education {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    School Name: {}", self.school_name)?;

        write!(f, "    Major(s): ")?;
        for major_item in self.major.iter() {
            write!(f, "{}, ", major_item)?;
        }
        writeln!(f)?;

        write!(f, "    Minor(s): ")?;
        for minor_item in self.minor.iter() {
            write!(f, "{}, ", minor_item)?;
        }
        writeln!(f)?;

        writeln!(f, "    Location: {}", self.location)?;

        write!(f, "    Courses: ")?;
        for course in self.coursework.iter() {
            write!(f, "{}, ", course)?;
        }
        writeln!(f)?;

        writeln!(f, "    GPA: {}", self.gpa)?;
        write!(f, "    Start Date: {}", self.education_start)?;

        if self.education_end.is_some() {
            write!(f, "    End Date: {}", self.education_end.unwrap())?;
        } else {
            writeln!(f, "    End Date: N/A")?;
        }

        Ok(())
    }
}

//__________________________________________

#[derive(Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    project_description: Vec<String>,

    project_start: Date,
    project_end: Option<Date>,
}

impl Project {
    fn new(
        project_name: String,
        project_description: Vec<String>,
        project_start: Date,
        project_end: Option<Date>,
    ) -> Self {
        Self {
            project_name,
            project_description,
            project_start,
            project_end,
        }
    }

    fn from(project: &Project) -> Self {
        Self {
            project_name: project.project_name.clone(),
            project_description: project.project_description.clone(),
            project_start: project.project_start.clone(),
            project_end: project.project_end.clone(),
        }
    }

    pub fn get_project_name(&self) -> &String {
        &self.project_name 
    }

    pub fn get_project_description(&self) -> &Vec<String> {
        &self.project_description 
    }

    pub fn get_project_start(&self) -> &Date {
        &self.project_start
    }

    pub fn get_project_end(&self) -> &Option<Date> {
        &self.project_end
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    {}", self.project_name)?;
        for item in self.project_description.iter() {
            writeln!(f, "        * {}", item)?;
        }
        writeln!(f)?;

        writeln!(f, "    Start Date: {}", self.project_start)?;

        if self.project_end.is_some() {
            writeln!(f, "    End Date: {}", self.project_end.unwrap())?;
        } else {
            writeln!(f, "    End Date: N/A")?;
        }

        Ok(())
    }
}

//----------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct CV {
    name: String,
    linked_in: Option<String>,
    website: Option<String>,
    skills: Skills,
    work_experience: Vec<WorkExperience>,
    education: Vec<Education>,
    projects: Vec<Project>
}

impl CV {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            linked_in: None,
            website: None,
            skills: Skills::new(),
            work_experience: Vec::new(),
            education: Vec::new(),
            projects: Vec::new(),
        }
    }

    // CV adders
    pub fn add_skill(&mut self, skill_name: String, category: Option<String>) {
        self.skills.add_skill(skill_name, category);
    }

    pub fn add_work_experience(
        &mut self,
        job_title: String,
        company_name: String,
        job_location: Option<String>,
        job_description: Vec<String>,
        job_start: Date,
        job_end: Option<Date>,
    ) {
        self.work_experience.push(WorkExperience::new(
            job_title,
            company_name,
            job_location,
            job_description,
            job_start,
            job_end,
        ));
    }

    pub fn add_education(
        &mut self,
        school_name: String,
        major: Vec<String>,
        location: String,
        minor: Vec<String>,
        coursework: Vec<String>,
        gpa: f32,
        education_start: Date,
        education_end: Option<Date>,
    ) {
        self.education.push(Education::new(
            school_name,
            major,
            location,
            minor,
            coursework,
            gpa,
            education_start,
            education_end,
        ));
    }

    pub fn add_project(
        &mut self,
        project_name: String,
        project_description: Vec<String>,
        project_start: Date,
        project_end: Option<Date>,
    ) {
        self.projects.push(Project::new(
            project_name,
            project_description,
            project_start,
            project_end,
        ));
    }

    // Functions to sort and create resume
    fn create_sorted_skill_list(
        &self,
        word_cloud: &WordCloud,
    ) -> Vec<(String, Vec<SortableResumeItem<String>>)> {
        let mut output: Vec<(String, Vec<SortableResumeItem<String>>)> = Vec::new();

        for category in self.skills.skill_tree.iter() {
            let mut category_skills: Vec<SortableResumeItem<String>> = Vec::new();

            for s in category.1 {
                category_skills.push(SortableResumeItem::new(
                    s.clone(),
                    word_cloud.get_word_score(s.clone()).get_word_weight(),
                ));
            }

            category_skills.sort_by_key(|skill| skill.point_value);

            output.push((category.0.clone(), category_skills));
        }

        output
    }

    fn create_sorted_work_experience_list(
        &mut self,
        word_cloud: &WordCloud,
    ) -> Vec<SortableResumeItem<WorkExperience>> {
        let mut output = Vec::new();

        for experience in self.work_experience.iter_mut() {
            // sort descriptions
            experience.job_description.sort_by_key(|description| {
                word_cloud
                    .get_word_score(description.clone())
                    .get_word_weight()
            });

            // Get total word weight
            let mut word_weight = 0;
            experience.job_description.iter().for_each(|description| {
                word_weight += word_cloud
                    .get_word_score(description.clone())
                    .get_word_weight()
            });
            word_weight += word_cloud
                .get_word_score(experience.job_title.clone())
                .get_word_weight();

            output.push(SortableResumeItem::new(
                WorkExperience::from(experience),
                word_weight,
            ));
        }
        output.sort_by_key(|experience| experience.point_value);

        output
    }

    fn create_sorted_education_list(
        &mut self,
        word_cloud: &WordCloud,
    ) -> Vec<SortableResumeItem<Education>> {
        let mut output = Vec::new();

        for education in self.education.iter_mut() {
            // sort relevate course
            education
                .coursework
                .sort_by_key(|course| word_cloud.get_word_score(course.clone()).get_word_weight());

            // Get total word weight
            let mut word_weight = 0;
            education.coursework.iter().for_each(|course| {
                word_weight += word_cloud.get_word_score(course.clone()).get_word_weight()
            });
            education.major.iter().for_each(|major| {
                word_weight += word_cloud.get_word_score(major.clone()).get_word_weight()
            });
            education.minor.iter().for_each(|minor| {
                word_weight += word_cloud.get_word_score(minor.clone()).get_word_weight()
            });

            output.push(SortableResumeItem::new(
                Education::from(education),
                word_weight,
            ));
        }
        output.sort_by_key(|education| education.point_value);

        output
    }

    fn create_sorted_project_list(
        &mut self,
        word_cloud: &WordCloud,
    ) -> Vec<SortableResumeItem<Project>> {
        let mut output = Vec::new();

        for project in self.projects.iter_mut() {
            // sort project descriptions
            project
                .project_description
                .sort_by_key(|description| word_cloud.get_word_score(description.clone()).get_word_weight());

            // get total word weight
            let mut word_weight = 0;
            project.project_description.iter().for_each(|description| {
                word_weight += word_cloud.get_word_score(description.clone()).get_word_weight();
            });


            output.push(SortableResumeItem::new(
                Project::from(project),
                word_weight
            ));
        }
        output.sort_by_key(|project| project.point_value);

        output
    }

    // Creates a resume with vectors sorted by word cloud score ratio
    pub fn generate_resume(&mut self, word_cloud: &WordCloud) -> Resume {
        let mut resume = Resume::new();

        // Add Skills to resume
        resume.skills = self.create_sorted_skill_list(&word_cloud);
        // Add Work experince to resume
        resume.work_experience = self.create_sorted_work_experience_list(&word_cloud);
        // Add Education to resume
        resume.education = self.create_sorted_education_list(&word_cloud);
        // Add Projects to resume
        resume.projects = self.create_sorted_project_list(&word_cloud);

        resume
    }

    // Saving and loading file
    pub fn save_to_file(&self, filename: String) {
        let saved_file = serde_json::to_string(self).unwrap();

        fs::write(filename, saved_file).expect("Unable to write to file");
    }

    pub fn load_from_file(&mut self, filename: String) {
        let mut contents = String::new();
        let _ = fs::File::open(filename)
            .unwrap()
            .read_to_string(&mut contents);
        *self = serde_json::from_str(&contents.as_str()).unwrap();
    }
}

impl Display for CV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Skills
        writeln!(f, "{}", self.skills)?;
        // Work Experience
        writeln!(f, "Work Experience:")?;
        for experience in self.work_experience.iter() {
            writeln!(f, "{}", experience)?;
        }
        // Education
        writeln!(f, "Education:")?;
        for edu in self.education.iter() {
            writeln!(f, "{}", edu)?;
        }
        // Projects
        writeln!(f, "Projects:")?;
        for project in self.projects.iter() {
            writeln!(f, "{}", project)?;
        }

        Ok(())
    }
}

pub struct SortableResumeItem<T> {
    pub sortable: T,
    point_value: i32,
}

impl<T> SortableResumeItem<T> {
    pub fn new(sortable: T, point_value: i32) -> Self {
        Self {
            sortable,
            point_value,
        }
    }
}

pub struct Resume {
    skills: Vec<(String, Vec<SortableResumeItem<String>>)>,
    work_experience: Vec<SortableResumeItem<WorkExperience>>,
    education: Vec<SortableResumeItem<Education>>,
    projects: Vec<SortableResumeItem<Project>>,
}

impl Resume {
    pub fn new() -> Self {
        Self {
            skills: Vec::new(),
            work_experience: Vec::new(),
            education: Vec::new(),
            projects: Vec::new(),
        }
    }

    pub fn get_skills(&self) -> &Vec<(String, Vec<SortableResumeItem<String>>)> {
        &self.skills
    }

    pub fn get_work_experience(&self) -> &Vec<SortableResumeItem<WorkExperience>> {
        &self.work_experience
    }

    pub fn get_education(&self) -> &Vec<SortableResumeItem<Education>> {
        &self.education
    }

    pub fn get_projects(&self) -> &Vec<SortableResumeItem<Project>> {
        &self.projects
    }
}

impl Display for Resume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Skills
        for category in self.skills.iter() {
            write!(f, "Category: {}\n    ", category.0)?;

            for skill in category.1.iter() {
                write!(f, "{}, ", skill.sortable)?;
            }

            writeln!(f)?;
        }
        // Work Experience
        writeln!(f, "Work Experience:")?;
        for experience in self.work_experience.iter() {
            writeln!(f, "{}", experience.sortable)?;
        }
        // Education
        writeln!(f, "Education:")?;
        for edu in self.education.iter() {
            writeln!(f, "{}", edu.sortable)?;
        }
        // Projects
        writeln!(f, "Projects:")?;
        for project in self.projects.iter() {
            writeln!(f, "{}", project.sortable)?;
        }

        Ok(())
    }
}
