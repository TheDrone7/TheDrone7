pub mod stack;
pub mod structure;

use stack::*;
use structure::Project;

use self::structure::{ProjectLink, ProjectStackItem};

pub fn project_good_anime() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "https://goodanime.thedrone7.dev/".to_string(),
    });
    links.push(ProjectLink {
        label: "Source".to_string(),
        href: "https://replit.com/@TheDrone7/GoodAnime".to_string(),
    });

    stack.push(stack_node_js());
    stack.push(stack_vue());
    stack.push(stack_typescript());
    stack.push(stack_graphql());

    Project {
        title: "GoodAnime".to_string(),
        description: "Random anime suggestions website. Updates every 30 seconds, with currently over 14,000 anime.".to_string(),
        links,
        stack,
    }
}

pub fn project_shieldbow() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "https://shieldbow.thedrone7.dev/".to_string(),
    });
    links.push(ProjectLink {
        label: "Source".to_string(),
        href: "https://github.com/TheDrone7/shieldbow".to_string(),
    });

    stack.push(stack_node_js());
    stack.push(stack_typescript());
    stack.push(stack_nuxt());

    Project {
        title: "Shieldbow".to_string(),
        description: "A very powerful NodeJS wrapper for the Riot Games API.".to_string(),
        links,
        stack,
    }
}

pub fn project_portfolio() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "/".to_string(),
    });
    links.push(ProjectLink {
        label: "Source".to_string(),
        href: "https://replit.com/@TheDrone7/TheDrone7".to_string(),
    });

    stack.push(stack_rust());
    stack.push(stack_tailwind());

    Project {
        title: "This website".to_string(),
        description: "My (TheDrone7's) portfolio website.".to_string(),
        links,
        stack,
    }
}

pub fn project_corona_jam() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "https://cjnine.thedrone7.repl.co/".to_string(),
    });
    links.push(ProjectLink {
        label: "Source".to_string(),
        href: "https://replit.com/@TheDrone7/cjnine".to_string(),
    });

    stack.push(stack_python());
    stack.push(stack_django());
    stack.push(stack_phaser());

    Project {
        title: "Coronajam".to_string(),
        description: "A very simple 2D platformer game that explains the COVID-19 prevention tips."
            .to_string(),
        links,
        stack,
    }
}

pub fn project_descord() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "https://deno.land/x/descord/".to_string(),
    });
    links.push(ProjectLink {
        label: "Source".to_string(),
        href: "https://github.com/TheDrone7/descord".to_string(),
    });

    stack.push(stack_deno());
    stack.push(stack_typescript());

    Project {
        title: "Descord (incomplete)".to_string(),
        description: "A stack_typescript()-based wrapper for the discord API with deno."
            .to_string(),
        links,
        stack,
    }
}

pub fn project_dronotes() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "#".to_string(),
    });

    stack.push(stack_node_js());
    stack.push(stack_typescript());
    stack.push(stack_tailwind());

    Project {
        title: "Dronotes (WIP)".to_string(),
        description:
            "A free web-based note-taking app with some extra features to boost your learning."
                .to_string(),
        links,
        stack,
    }
}

pub fn project_mod_dashboard() -> Project {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View".to_string(),
        href: "#".to_string(),
    });

    stack.push(stack_node_js());
    stack.push(stack_express());
    stack.push(stack_graphql());
    stack.push(stack_next());
    stack.push(stack_typescript());
    stack.push(stack_emotion());

    Project {
        title: "Replit moderation dashboard (WIP)".to_string(),
        description: "The moderation dashboard for handling reports, and taking other actions for the replit site moderators.".to_string(),
        links,
        stack,
    }
}

pub fn get_project_list() -> Vec<Project> {
    let mut projects: Vec<Project> = Vec::new();
    projects.push(project_good_anime());
    projects.push(project_shieldbow());
    projects.push(project_portfolio());
    projects.push(project_corona_jam());
    projects.push(project_descord());
    projects.push(project_dronotes());
    projects.push(project_mod_dashboard());
    projects
}
