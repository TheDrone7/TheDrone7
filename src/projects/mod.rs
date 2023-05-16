pub mod stack;
pub mod structure;

use stack::*;
use structure::Project;

use self::structure::{ProjectLink, ProjectStackItem};

const GOOD_ANIME: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "https://goodanime.thedrone7.dev/",
    });
    links.push(ProjectLink {
        label: "Source",
        href: "https://replit.com/@TheDrone7/GoodAnime",
    });

    stack.push(NODE_JS);
    stack.push(VUE);
    stack.push(TYPESCRIPT);
    stack.push(GRAPHQL);

    Project {
        title: "GoodAnime",
        description: "Random anime suggestions website. Updates every 30 seconds, with currently over 14,000 anime.",
        links,
        stack,
    }
};

const SHIELDBOW: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "https://shieldbow.thedrone7.dev/",
    });
    links.push(ProjectLink {
        label: "Source",
        href: "https://github.com/TheDrone7/shieldbow",
    });

    stack.push(NODE_JS);
    stack.push(TYPESCRIPT);
    stack.push(NUXT_JS);

    Project {
        title: "Shieldbow",
        description: "A very powerful NodeJS wrapper for the Riot Games API.",
        links,
        stack,
    }
};

const PORTFOLIO: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "/",
    });
    links.push(ProjectLink {
        label: "Source",
        href: "https://replit.com/@TheDrone7/TheDrone7",
    });

    stack.push(RUST);
    stack.push(TAILWIND);

    Project {
        title: "This website",
        description: "My (TheDrone7's) portfolio website.",
        links,
        stack,
    }
};

const CORONA_JAM: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "https://cjnine.thedrone7.repl.co/",
    });
    links.push(ProjectLink {
        label: "Source",
        href: "https://replit.com/@TheDrone7/cjnine",
    });

    stack.push(PYTHON);
    stack.push(DJANGO);
    stack.push(PHASER);

    Project {
        title: "Coronajam",
        description: "A very simple 2D platformer game that explains the COVID-19 prevention tips.",
        links,
        stack,
    }
};

const DESCORD: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "https://deno.land/x/descord/",
    });
    links.push(ProjectLink {
        label: "Source",
        href: "https://github.com/TheDrone7/descord",
    });

    stack.push(DENO);
    stack.push(TYPESCRIPT);

    Project {
        title: "Descord (incomplete)",
        description: "A typescript-based wrapper for the discord API with deno.",
        links,
        stack,
    }
};

const DRONOTES: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "#",
    });

    stack.push(NODE_JS);
    stack.push(TYPESCRIPT);
    stack.push(TAILWIND);

    Project {
        title: "Dronotes (WIP)",
        description:
            "A free web-based note-taking app with some extra features to boost your learning.",
        links,
        stack,
    }
};

const MOD_DASHBOARD: Project = {
    let mut links: Vec<ProjectLink> = Vec::new();
    let mut stack: Vec<ProjectStackItem> = Vec::new();

    links.push(ProjectLink {
        label: "View",
        href: "#",
    });

    stack.push(NODE_JS);
    stack.push(EXPRESS);
    stack.push(GRAPHQL);
    stack.push(NEXT_JS);
    stack.push(TYPESCRIPT);
    stack.push(EMOTION);

    Project {
        title: "Replit moderation dashboard (WIP)",
        description: "The moderation dashboard for handling reports, and taking other actions for the replit site moderators.",
        links,
        stack,
    }
};

pub const PROJECT_LIST: Vec<Project> = {
    let mut projects: Vec<Project> = Vec::new();
    projects.push(GOOD_ANIME);
    projects.push(SHIELDBOW);
    projects.push(PORTFOLIO);
    projects.push(CORONA_JAM);
    projects.push(DESCORD);
    projects.push(DRONOTES);
    projects.push(MOD_DASHBOARD);
    projects
};
