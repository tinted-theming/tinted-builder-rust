default:
    @just --list

test project="" *args:
    {{ if project == "" { "cargo test --workspace" } else { "cargo test -p " + project } }} {{args}}

fmt:
    cargo fmt
    alejandra .

lint:
    cargo clippy

clean:
    rm -rf target/
