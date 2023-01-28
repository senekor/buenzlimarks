use uuid::Uuid;

use crate::models::bookmark::Bookmark;

use super::BuenzlimarksDb;

// user(id), pages, widgets, bookmarks(name, link)
#[allow(clippy::type_complexity)]
static DATA: &[(&str, &[&[&[(&str, &str)]]])] = &[(
    crate::models::user::DEV_USER,
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

pub fn insert_seeds(db: &(dyn BuenzlimarksDb + Send + Sync)) {
    for user in DATA {
        let user_id = user.0;
        for page in user.1 {
            let p_id = Uuid::new_v4().to_string();
            for widget in page.iter() {
                let w_id = Uuid::new_v4().to_string();
                for (name, link) in widget.iter().copied() {
                    let bm_id = Uuid::new_v4().to_string();
                    let bookmark = Bookmark {
                        id: bm_id,
                        name: name.into(),
                        link: link.into(),
                        widget_id: w_id.clone(),
                    };
                    db.insert_bookmark(user_id, &p_id, &w_id, &bookmark)
                        .unwrap();
                }
            }
        }
    }
}
