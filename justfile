_default:
    @just --list

domain := "github.com"
img_name := domain / "senekor/buenzlimarks"

# run the server, watching for changes
watch:
    @killall buenzlimarks &> /dev/null || true
    cd server && cargo bin cargo-watch -x run

# run the server
run *args:
    @cd server && cargo run -q -- {{ args }}

# run the web app devel server, watching for changes
app-watch:
    @killall trunk &> /dev/null || true
    cd app && cargo bin trunk serve --open

# initialize a new development database
db-reset:
    cd server && cargo run --bin db_reset

# render the documentation book, watching for changes
book-watch:
    @killall mdbook &> /dev/null || true
    cd docs && cargo bin mdbook serve --port 5000

# render d2 diagrams, watching for changes
diagrams-watch:
    cargo bin watchexec --debounce 1000 \
        --emit-events-to file \
        --watch docs/diagrams \
        --restart ./devel/render_diagrams.sh

# start a terminal workspace for development
zellij:
    zellij --layout devel/zellij.kdl
    @killall buenzlimarks &> /dev/null || true
    @killall trunk &> /dev/null || true
    @killall mdbook &> /dev/null || true

podman-build:
    podman build --platform linux/amd64,linux/arm64 --manifest {{ img_name }} .

podman-run:
    podman run -it {{ img_name }}
