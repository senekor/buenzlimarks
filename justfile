_default:
    @just --list

set dotenv-load

# run the server
server-run:
    cd server && cargo run

# run the web app dev server
app-run:
    cd app && pnpm dev

# initialize a new development database
db-reset:
    cd server && cargo run --bin db_reset

# render the given diagram
diagram-render diagram:
    d2 --watch --layout=elk --pad=0 --theme=1 \
        docs/arc42/d2/{{diagram}}.d2 \
        docs/arc42/diagrams/{{diagram}}.svg
