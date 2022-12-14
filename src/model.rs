use std::{collections::HashMap, error::Error, path::Path, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::config::{Config, URLs};

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
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let content = serde_yaml::to_string(&self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

pub struct Projects {
    data: HashMap<String, Project>,
    projects_path: String,
}

impl Projects {
    pub fn new(projects_path: &str) -> Result<Projects, Box<dyn Error>> {
        let mut projects = Projects {
            data: HashMap::new(),
            projects_path: projects_path.to_string(),
        };
        projects.load()?;
        Ok(projects)
    }
    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let mut data = HashMap::new();
        let paths = std::fs::read_dir(&self.projects_path).expect(&format!(
            "Failed to read projects directory: {}",
            self.projects_path
        ));
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
    pub fn save(&mut self, project: &Project) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.projects_path).join(&project.id);
        project.save(&path)?;
        self.load()?;
        Ok(())
    }
}

pub struct AppState {
    pub projects: Mutex<Projects>,
    pub urls: URLs,
}

impl AppState {
    pub fn new(config: Config) -> Result<AppState, Box<dyn Error>> {
        Ok(AppState {
            projects: Mutex::new(Projects::new(&config.projects_path)?),
            urls: config.urls,
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
    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        let mut lock = self.projects.lock().unwrap();
        lock.load()?;
        Ok(())
    }
    pub fn save(&self, project: Project) -> Result<(), Box<dyn Error>> {
        let mut lock = self.projects.lock().unwrap();
        lock.save(&project)?;
        Ok(())
    }
    pub fn add_items(&self, project_id: &str, items: Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut lock = self.projects.lock().unwrap();
        let mut project = lock.get(project_id).unwrap().clone();
        project.items.extend(items);
        lock.save(&project)?;
        Ok(())
    }
}
