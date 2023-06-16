use buenzlimarks::{
    db::{self, config::DbConfig, DbTrait},
    models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget},
};
use clap::Parser;

fn insert_seeds(db: &(dyn DbTrait + Send + Sync)) {
    // user(id), pages, widgets, bookmarks(name, url)
    #[allow(clippy::type_complexity)]
    let data: &[(User, &[&[&[(&str, &str)]]])] = &[(
        User::dev(),
        &[&[
            &[
                (
                    "Requirements",
                    "https://github.com/users/remlse/projects/1/views/6",
                ),
                (
                    "Prioritization",
                    "https://github.com/users/remlse/projects/1/views/7",
                ),
                (
                    "Tasks",
                    "https://github.com/users/remlse/projects/1/views/2",
                ),
            ],
            &[
                ("YouTube", "https://youtube.com"),
                ("Rust std docs", "https://std.rs"),
            ],
        ]],
    )];

    for user in data {
        let user_id = user.0.id.clone();
        db.insert_user(user.0.clone()).unwrap();
        for page in user.1 {
            let p_id = Id::random();
            db.insert_page(&user_id, Page { id: p_id.clone() }).unwrap();
            for widget in page.iter() {
                let w_id = Id::random();
                db.insert_widget(
                    &user_id,
                    Widget {
                        id: w_id.clone(),
                        page_id: p_id.clone(),
                    },
                )
                .unwrap();
                for (name, url) in widget.iter().copied() {
                    let bm_id = Id::random();
                    let bookmark = Bookmark {
                        id: bm_id,
                        name: name.into(),
                        url: url.into(),
                        widget_id: w_id.clone(),
                    };
                    db.insert_bookmark(&user_id, bookmark).unwrap();
                }
            }
        }
    }
}

fn main() {
    let config = DbConfig::parse();

    std::fs::remove_dir_all(&config.db_root_dir).ok();

    let db = db::get(&config);
    insert_seeds(db.as_ref());
}
