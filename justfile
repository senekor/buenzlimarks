_default:
    @just --list

# run the server, watching for changes
watch:
    @killall buenzlimarks &> /dev/null || true
    cd server && cargo watch -x run

# run the server
run *args:
    cd server && cargo run -- {{ args }}

# run the web app dev server, watching for changes
app-watch:
    @killall trunk &> /dev/null || true
    cd app && trunk serve --open

# initialize a new development database
db-reset:
    cd server && cargo run --bin db_reset

# render the documentation book, watching for changes
book-watch:
    @killall mdbook &> /dev/null || true
    cd docs && mdbook serve --port 5000

# render d2 diagrams, watching for changes
diagrams-watch:
    watchexec --debounce 1000 \
        --emit-events-to file \
        --watch docs/diagrams \
        --restart ./dev/render_diagrams.sh

# start a terminal workspace for development
zellij:
    zellij --layout dev/zellij.kdl
    @killall buenzlimarks &> /dev/null || true
    @killall trunk &> /dev/null || true
    @killall mdbook &> /dev/null || true
