#[cfg(debug_assertions)]
static BUILD_CMD: &str = "build-dev";
#[cfg(not(debug_assertions))]
static BUILD_CMD: &str = "build";

fn main() {
    // println!("cargo:warning= Nifty trick to debug build scripts! ");
    println!("cargo:rerun-if-changed=../app/src");
    std::process::Command::new("pnpm")
        .args(["--frozen-lockfile", "--dir", "../app", "install"])
        .output()
        .expect("failed to install frontend dependencies");
    std::process::Command::new("pnpm")
        .args(["--dir", "../app", BUILD_CMD])
        .output()
        .expect("failed to build frontend");
}
