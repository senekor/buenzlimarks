fn main() {
    println!("cargo:rerun-if-changed=../app/src");
    std::process::Command::new("pnpm")
        .args(["--dir", "../app", "install"])
        .output()
        .expect("failed to install frontend dependencies");
    std::process::Command::new("pnpm")
        .args(["--dir", "../app", "build"])
        .output()
        .expect("failed to build frontend");
}
