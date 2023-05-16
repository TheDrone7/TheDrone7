pub struct ProjectLink {
    label: &str,
    href: &str,
}

pub struct ProjectStackItem {
    name: &str,
    desc: &str,
    image: &str,
}

pub struct Project {
    title: &str,
    description: &str,
    links: Vec<ProjectLink>,
    stack: Vec<ProjectStackItem>,
}
