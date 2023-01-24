use std::path::PathBuf;

fn main() {
    let db_root_dir = PathBuf::from(std::env::var("FS_DB_ROOT_DIR").expect("DB dir not found"));
    let seed_bookmarks_dir = db_root_dir.join("users/dev/pages/p0/widgets/w0/bookmarks");
    std::fs::create_dir_all(&seed_bookmarks_dir).expect("failed to create bookmarks dir");

    for (i, (name, link)) in [
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
        ("YouTube", "https://youtube.com"),
        ("Rust std docs", "https://std.rs"),
    ]
    .into_iter()
    .enumerate()
    {
        std::fs::write(
            seed_bookmarks_dir.join(format!("b{i}")),
            format!("{{\"id\":\"b{i}\",\"name\":\"{name}\",\"link\":\"{link}\"}}"),
        )
        .unwrap();
    }
}
