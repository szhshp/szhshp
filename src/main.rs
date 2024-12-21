use std::fs::File;
use std::io::Write;

use anyhow::Result;
use tera::{Context, Tera};

struct Site {
    url: String,
    active: bool,
}

fn main() -> Result<()> {
    let tera = Tera::new("*.tmpl")?;
    let mut context = Context::new();

    let my_sites: Vec<Site> = vec![
        Site {
            url: "https://szhshp.org".to_string(),
            active: true,
        },
        Site {
            url: "https://note.szhshp.org".to_string(),
            active: false,
        },
        Site {
            url: "https://music.szhshp.org".to_string(),
            active: false,
        },
        Site {
            url: "https://titan.szhshp.org".to_string(),
            active: false,
        },
        Site {
            url: "https://memo.szhshp.org".to_string(),
            active: false,
        },
    ];

    let tags: Vec<&str> = vec![
        "rust",
        "typescript",
        "react",
        "nextjs",
        "playwright",
        // "remixjs",
        // "qwik",
        "tailwind",
        "aws",
        "azure",
        "electron",
        "graphql",
        "jest",
        "materialUI",
        "mongodb",
        "mysql",
        "nodejs",
        "photoshop",
        "redux",
        "vite",
        "sass",
        "sqlserver",
    ];

    context.insert(
        "my_sites",
        &my_sites
            .iter()
            .map(|site| {
                let site_md = if site.active {
                    format!("- [{}]({})", site.url, site.url)
                } else {
                    format!("- ~~[{}]({})~~", site.url, site.url)
                };
                site_md
            })
            .collect::<Vec<String>>()
            .join(" \n"),
    );
    context.insert(
        "tags",
        &tags
            .iter()
            .map(|tag| '`'.to_string() + tag + "`")
            .collect::<Vec<String>>()
            .join(" "),
    );

    let readme = tera.render("README.tmpl", &context)?;
    let mut file = File::create("README.md")?;
    file.write_all(readme.as_bytes())?;
    Ok(())
}
