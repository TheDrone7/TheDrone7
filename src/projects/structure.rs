#[derive(Clone, PartialEq)]
pub struct ProjectLink {
    pub label: String,
    pub href: String,
}

#[derive(Clone, PartialEq)]
pub struct ProjectStackItem {
    pub name: String,
    pub desc: String,
    pub image: String,
}

#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub links: Vec<ProjectLink>,
    pub stack: Vec<ProjectStackItem>,
}
