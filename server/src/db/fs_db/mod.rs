use std::path::PathBuf;

use crate::entities::bookmark::Bookmark;

use super::BuenzlimarksDatabase;

#[derive(Debug, Clone)]
pub struct FileSystemDatabase {
    root_dir: PathBuf,
}

impl FileSystemDatabase {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }
}

impl BuenzlimarksDatabase for FileSystemDatabase {
    fn get_bookmarks(&self, user_id: &str) -> Vec<Bookmark> {
        std::fs::read_dir(
            self.root_dir
                .join("users/dev/pages/p0/widgets/w0/bookmarks"),
        )
        .unwrap()
        .map(|dir_entry| {
            let path = dir_entry.unwrap().path();
            let s = std::fs::read_to_string(path).unwrap();
            serde_json::from_str(&s).unwrap()
        })
        .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::super::BuenzlimarksDatabase;
    use super::*;

    fn new_db() -> FileSystemDatabase {
        // Bad practice !!
        // Do not rely on en env var and or a pre seeded dev db for tests.
        // Instead, generate a temporary directory and fill it with test
        // specific seed data.
        FileSystemDatabase::new(PathBuf::from(
            std::env::var("FS_DB_ROOT_DIR").expect("DB dir not found"),
        ))
    }

    #[test]
    fn get_bookmarks() {
        let db = new_db();
        assert_eq!(
            db.get_bookmarks("dev"),
            vec![
                Bookmark {
                    id: "b0".into(),
                    name: "Requirements".into(),
                    link: "https://github.com/users/remlse/projects/1/views/6".into(),
                },
                Bookmark {
                    id: "b1".into(),
                    name: "Prioritization".into(),
                    link: "https://github.com/users/remlse/projects/1/views/7".into(),
                },
                Bookmark {
                    id: "b2".into(),
                    name: "Tasks".into(),
                    link: "https://github.com/users/remlse/projects/1/views/2".into(),
                },
                Bookmark {
                    id: "b3".into(),
                    name: "YouTube".into(),
                    link: "https://youtube.com".into(),
                },
                Bookmark {
                    id: "b4".into(),
                    name: "Rust std docs".into(),
                    link: "https://std.rs".into(),
                }
            ]
        )
    }
}
