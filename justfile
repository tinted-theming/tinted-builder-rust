default:
    @just --list

build project:
    cargo build -p {{project}} --release

build-wasm:
    cargo build -p tinted-builder --target wasm32-wasip2

test project="" *args:
    {{ if project == "" { "cargo test --workspace" } else { "cargo test -p " + project } }} {{args}}

test-wasm *args:
    cargo test -p tinted-builder {{args}}

fmt:
    cargo fmt
    alejandra .

lint:
    cargo clippy

clean:
    rm -rf target/
