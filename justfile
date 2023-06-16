_default:
    @just --list

# run the server, watching for changes
watch:
    cd server && cargo watch -x run

# run the server
run *args:
    cd server && cargo run -- {{ args }}

# run the web app dev server
app-run:
    cd app && pnpm dev

# initialize a new development database
db-reset:
    cd server && cargo run --bin db_reset

# render the given diagram
render-diagram diagram:
    d2 --watch --layout=elk --pad=32 \
        docs/arc42/d2/{{diagram}}.d2 \
        docs/arc42/diagrams/{{diagram}}.svg

zellij:
    zellij --layout dev/zellij.kdl
