use uuid::Uuid;

use crate::models::{bookmark::Bookmark, page::Page, widget::Widget};

use super::DbTrait;

// user(id), pages, widgets, bookmarks(name, url)
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

pub fn insert_seeds(db: &(dyn DbTrait + Send + Sync)) {
    for user in DATA {
        let user_id = user.0;
        for page in user.1 {
            let p_id = Uuid::new_v4().to_string();
            db.insert_page(user_id, &Page { id: p_id.clone() }).unwrap();
            for widget in page.iter() {
                let w_id = Uuid::new_v4().to_string();
                db.insert_widget(
                    user_id,
                    &Widget {
                        id: w_id.clone(),
                        page_id: p_id.clone(),
                    },
                )
                .unwrap();
                for (name, url) in widget.iter().copied() {
                    let bm_id = Uuid::new_v4().to_string();
                    let bookmark = Bookmark {
                        id: bm_id,
                        name: name.into(),
                        url: url.into(),
                        widget_id: w_id.clone(),
                    };
                    db.insert_bookmark(user_id, &bookmark).unwrap();
                }
            }
        }
    }
}
