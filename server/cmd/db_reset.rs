use buenzlimarks_server::db::{self, config::DbConfig, Database};
use clap::Parser;
use models::{AuthProvider, Bookmark, Id, Page, Settings, User, Widget};

type NameSeed<'a> = &'a str;
type UrlSeed<'a> = &'a str;

type UserSeed<'a> = (User, Settings, Vec<PageSeed<'a>>);
type PageSeed<'a> = (NameSeed<'a>, Vec<WidgetSeed<'a>>);
type WidgetSeed<'a> = (NameSeed<'a>, Vec<BookmarkSeed<'a>>);
type BookmarkSeed<'a> = (NameSeed<'a>, UrlSeed<'a>);

fn insert_seeds(db: &Database) {
    let seed_data: Vec<UserSeed> = vec![(
        User {
            id: "buenzli".into(),
            provider: AuthProvider::Dev,
        },
        Settings {
            name: "Bünzli".into(),
        },
        vec![
            (
                "Seed page",
                vec![
                    // 1. widget
                    (
                        "wandern",
                        vec![
                            // bookmarks
                            (
                                "Requirements",
                                "https://github.com/users/senekor/projects/1/views/6",
                            ),
                            (
                                "Prioritization",
                                "https://github.com/users/senekor/projects/1/views/7",
                            ),
                            (
                                "Tasks",
                                "https://github.com/users/senekor/projects/1/views/2",
                            ),
                        ],
                    ),
                    // 2. widget
                    (
                        "Sozialversicherungen",
                        vec![
                            ("YouTube", "https://youtube.com"),
                            ("Rust std docs", "https://std.rs"),
                        ],
                    ),
                ],
            ),
            (
                "Second page",
                vec![(
                    "Tastaturen",
                    vec![
                        ("beekeeb", "https://beekeeb.com"),
                        ("ZSA", "https://zsa.io"),
                    ],
                )],
            ),
        ],
    )];

    for (user, settings, pages) in seed_data {
        db.insert_user(&user, settings).unwrap();
        for (page_name, widgets) in pages {
            let p_id = Id::random();
            db.insert_entity(
                &user,
                Page {
                    id: p_id.clone(),
                    name: page_name.into(),
                },
            )
            .unwrap();
            for (widget_name, bookmarks) in widgets {
                let w_id = Id::random();
                db.insert_entity(
                    &user,
                    Widget {
                        id: w_id.clone(),
                        name: widget_name.into(),
                        page_id: p_id.clone(),
                    },
                )
                .unwrap();
                for (name, url) in bookmarks {
                    let bm_id = Id::random();
                    let bookmark = Bookmark {
                        id: bm_id,
                        name: name.into(),
                        url: url.into(),
                        widget_id: w_id.clone(),
                    };
                    db.insert_entity(&user, bookmark).unwrap();
                }
            }
        }
    }
}

fn main() {
    let config = DbConfig::parse();

    std::fs::remove_dir_all(&config.db_dir).ok();

    let db = db::Database::get(&config);
    insert_seeds(&db);
}
