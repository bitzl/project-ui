use std::{path::Path, collections::HashMap, sync::Mutex, error::Error};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub manifest_id: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl Project {
    pub fn load(path: &Path) -> Result<Project, Box<dyn Error>> {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let project: Project = serde_yaml::from_str(&content)?;
                Ok(project)
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub struct Projects {
    data: HashMap<String, Project>,
    data_path: String,
}

impl Projects {
    pub fn new(data_path: &str) -> Result<Projects, Box<dyn Error>> {
        let mut projects = Projects {
            data: HashMap::new(),
            data_path: data_path.to_string(),
        };
        projects.load()?;
        Ok(projects)
    }
    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let mut data = HashMap::new();
        let paths = std::fs::read_dir(&self.data_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if !path.is_file() {
                continue;
            }
            println!("Loading project: {:?}", path);
            let project = Project::load(&path)?;
            data.insert(project.id.clone(), project);
        }
        self.data = data;
        Ok(())
    }
    pub fn get(&self, project_id: &str) -> Option<&Project> {
        self.data.get(project_id)
    }
    pub fn list(&self) -> Vec<&Project> {
        self.data.values().into_iter().collect()
    }
}


pub struct AppState {
    pub projects: Mutex<Projects>,
}

impl AppState {
    pub fn new(data_path: &str) -> Result<AppState, Box<dyn Error>> {
        Ok(AppState {
            projects: Mutex::new(Projects::new(data_path)?),
        })
    }
    pub fn get(&self, project_id: &str) -> Option<Project> {
        let lock = self.projects.lock().unwrap();
        lock.get(project_id).cloned()
    }
    pub fn list(&self) -> Vec<Project> {
        let lock = self.projects.lock().unwrap();
        lock.list().into_iter().map(|p| p.clone()).collect()
    }
    pub fn update(&self) {
        let mut lock = self.projects.lock().unwrap();
        lock.load().unwrap();
    }
}