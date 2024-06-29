use dioxus::{prelude::*, router::components};

#[derive(PartialEq, Clone, Debug)]
pub struct Project<'a> {
    apn: String,
    project_name: String,
    date: String,
    assessed_value: &'a f32,
    acres: &'a f32,
    description: String,
    stage: String,
    project_progress: &'a f32,
    site_assessment: String,
    longitude: &'a f64,
    latitude: &'a f64,
}

impl<'a> Project<'a> {
    // The 'new' method takes ownership of the String arguments and borrows the f32 arguments.
    fn new(
        apn: String,
        project_name: String,
        date: String,
        assessed_value: &'a f32,
        acres: &'a f32,
        description: String,
        stage: String,
        project_progress: &'a f32,
        site_assessment: String,
        longitude: &'a f64,
        latitude: &'a f64,
    ) -> Self {
        Project {
            apn,
            project_name,
            date,
            assessed_value,
            acres,
            description,
            stage,
            project_progress,
            site_assessment,
            longitude,
            latitude,
        }
    }
}

#[component]
pub fn EnergyDashboard() -> Element {
    let temp_projects: Vec<Vec<Project>> = Vec::new();
    let mut projects = use_signal(|| temp_projects);
    // Example of adding a default project for demonstration
    // This can be removed or modified as needed
    if projects.len() == 0 {
        projects.push(vec![
            Project {
                apn: String::from("12345"),
                project_name: String::from("Example Project"),
                date: String::from("2024-01-01"),
                assessed_value: &350000.00,
                acres: &10.0,
                description: String::from("Example Description"),
                stage: String::from("Planning"),
                project_progress: &50.0,
                site_assessment: String::from("https://hospitalprogram.my.canva.site/"),
                longitude: &127.3567775,
                latitude: &36.54433334,
            },
            Project {
                apn: String::from("12345"),
                project_name: String::from("Example Project"),
                date: String::from("2024-01-01"),
                assessed_value: &350000.00,
                acres: &10.0,
                description: String::from("Example Description"),
                stage: String::from("Planning"),
                project_progress: &50.0,
                site_assessment: String::from("https://hospitalprogram.my.canva.site/"),
                longitude: &127.3567775,
                latitude: &36.54433334,
            },
        ]);
        projects.push(vec![Project {
            apn: String::from("12345"),
            project_name: String::from("Example Project"),
            date: String::from("2024-01-01"),
            assessed_value: &350000.00,
            acres: &10.0,
            description: String::from("Example Description"),
            stage: String::from("Planning"),
            project_progress: &50.0,
            site_assessment: String::from("https://hospitalprogram.my.canva.site/"),
            longitude: &127.3567775,
            latitude: &36.54433334,
        }]);
    };
    rsx! {
        {projects.iter().map(|project_set| rsx! {
            table {class: "fl-table",
            h4 {"Adelanto"}
                        thead {
                            tr {
                                th {"Project Name"}
                                th { "APN" }
                                th { "Date" }
                                th { "Assessed Value"}
                                th { "Acres" }
                                th { "(long, lat)" }
                                th { "Stage" }
                                th { "Project Progress" }
                                th { "Project Site Assessment" }
                            }
                        }
            {project_set.iter().map(|project| rsx! {
                        tr {
                            td { "{project.project_name}"}
                            td { "{project.apn}" }
                            td { "{project.date}" }
                            td { "{project.assessed_value}"}
                            td { "{project.acres}" }
                            td { "({project.longitude},{project.latitude})" }
                            td { "{project.stage}" }
                            td { "{project.project_progress}" }
                            td { a { href: "{project.site_assessment}", "Report" }}
                        }
                })}
            }
        })}
    }
}

#[component]
pub fn CountingNumbers() -> Element {
    todo!()
}
