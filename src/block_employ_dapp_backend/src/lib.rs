#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[macro_use]
use candid::{Decode, Encode, Result};
use serde::{Deserialize, Serialize};
extern crate serde;
use std::collections::HashMap;

// Define a struct to represent a job posting
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JobPosting {
    id: u64,
    title: String,
    description: String,
    required_skills: Vec<String>,
}

// Define a struct to represent an employee
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Employee {
//     id: u64,
//     name: String,
//     skills: Vec<String>,
// }
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
struct Employee {
    id: u64,
    name: String,
    skills: Vec<String>,
}


// Define a struct to represent a company
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Company {
    id: u64,
    name: String,
    job_postings: Vec<JobPosting>,
}

// Define a struct to represent the government
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Government;

// Define a struct to represent the job matching system
#[derive(Debug)]
struct JobMatcher {
    companies: Vec<Company>,
    employees: Vec<Employee>,
}

impl JobMatcher {
    // Method to add a company to the job matcher
    fn add_company(&mut self, company: Company) {
        self.companies.push(company);
    }

    // Method to add an employee to the job matcher
    fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    // Method to match employees with suitable jobs
    fn match_jobs(&self) -> HashMap<&Employee, Vec<&JobPosting>> {
        let mut job_matches: HashMap<&Employee, Vec<&JobPosting>> = HashMap::new();

        for employee in &self.employees {
            let mut matches: Vec<&JobPosting> = Vec::new();

            for company in &self.companies {
                for job_posting in &company.job_postings {
                    if job_posting.required_skills.iter().all(|skill: &String| employee.skills.contains(skill))
                    {
                        matches.push(job_posting);
                    }
                }
            }

            job_matches.insert(employee, matches);
        }

        job_matches
    }
}

fn main() {
    // Sample data initialization
    let mut job_matcher = JobMatcher {
        companies: Vec::new(),
        employees: Vec::new(),
    };

    let company1 = Company {
        id: 1,
        name: "Company A".to_string(),
        job_postings: vec![
            JobPosting {
                id: 1,
                title: "Software Engineer".to_string(),
                description: "Develop software applications".to_string(),
                required_skills: vec!["Programming".to_string(), "Problem Solving".to_string()],
            },
            JobPosting {
                id: 2,
                title: "Data Scientist".to_string(),
                description: "Analyze and interpret complex datasets".to_string(),
                required_skills: vec!["Data Analysis".to_string(), "Statistics".to_string()],
            },
        ],
    };

    let employee1 = Employee {
        id: 1,
        name: "John Doe".to_string(),
        skills: vec!["Programming".to_string(), "Problem Solving".to_string()],
    };

    let employee2 = Employee {
        id: 2,
        name: "Jane Doe".to_string(),
        skills: vec!["Data Analysis".to_string(), "Statistics".to_string()],
    };

    job_matcher.add_company(company1.clone());
    job_matcher.add_employee(employee1.clone());
    job_matcher.add_employee(employee2.clone());

    // Match jobs for employees
    let job_matches = job_matcher.match_jobs();
    for (employee, jobs) in job_matches.iter() {
        println!("Employee: {}", employee.name);
        println!("Matched Jobs:");
        for job in jobs {
            println!("  - {}", job.title);
        }
        println!();
    }

}






