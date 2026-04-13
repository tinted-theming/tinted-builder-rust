# WASI Component Model Support

The `tinted-builder` library can be built as a [WASI Component
Model](https://component-model.bytecodealliance.org/) component, allowing it to
be used from any language with a WASI-compatible runtime (Go, Python,
JavaScript, C#, etc.).

## Prerequisites

This project uses a [Nix flake](../flake.nix) to provide the required tooling.
Enter the dev shell:

```sh
nix develop
```

This gives you:

- Rust toolchain with the `wasm32-wasip2` target
- `just` - Command runner
- `wasmtime` - WASI runtime
- `wasm-tools` - Inspect and manipulate `.wasm` files

If you're not using Nix, install manually:

```sh
rustup target add wasm32-wasip2
cargo install wasmtime-cli wasm-tools
```

## Building

```sh
just build-wasm
```

The component will be at `target/wasm32-wasip2/debug/tinted_builder.wasm` (or
`release/` with `--release`).

### Inspecting the component

```sh
# View the WIT interface embedded in the component
wasm-tools component wit target/wasm32-wasip2/debug/tinted_builder.wasm

# Validate the component
wasm-tools validate target/wasm32-wasip2/debug/tinted_builder.wasm --features component-model
```

## Architecture

The WASI component is defined by three pieces:

| File                           | Purpose                                                             |
| ------------------------------ | ------------------------------------------------------------------- |
| `tinted-builder/wit/world.wit` | WIT interface definition - the contract consumers code against      |
| `tinted-builder/src/wasm.rs`   | Implements the WIT interfaces by delegating to the existing library |
| `tinted-builder/src/lib.rs`    | Gates the WASM code behind `#[cfg(target_arch = "wasm32")]`         |

The library compiles normally as an `rlib` for native targets. The `cdylib` +
WASI component exports only activate when targeting `wasm32-wasip2`.

## WIT Interface

The component exports five interfaces under the `tinted-theming:tinted-builder`
package:

### `types`

Shared type definitions used across all interfaces:

- `scheme-system` - enum: `base16`, `base24`, `tinted8`
- `scheme-variant` - enum: `dark`, `light`
- `color-name` - enum: `black`, `red`, `green`, `yellow`, `blue`, `magenta`,
  `cyan`, `white`, `orange`, `gray`, `brown`, `other`
- `color-variant` - enum: `dim`, `normal`, `bright`
- `color` - record with hex, RGB, RGB16, and decimal representations
- `scheme-metadata` - system, name, slug, author, description, variant
- `tinted8-scheme-metadata` - extended metadata with theme-author, family,
  style, supports-styling-spec
- `palette-entry` - key-color pair

### `colors`

Color construction and derivation:

- `create(hex, name?, variant?) -> color` - create a color from a hex string
  (e.g. `"ff00ff"` or `"#ffcc00"`)
- `to-variant(color, target-variant) -> color` - derive a dim or bright variant
  from a color
- `to-color(color, target-name) -> color` - derive supplemental colors (e.g.
  orange from yellow)

### `schemes`

Scheme parsing with the `scheme` resource:

- `scheme.parse(yaml) -> scheme` - parse a YAML string, auto-detecting the
  system (base16/base24/tinted8)
- `scheme.metadata() -> scheme-metadata` - get scheme metadata
- `scheme.palette() -> list<palette-entry>` - get palette as key-color pairs

### `renderer`

Template rendering:

- `render(scheme, template-content) -> string` - render a Mustache-style
  template with a scheme's colors

### `tinted8`

Tinted8-specific functionality with the `tinted8-scheme` resource:

- `tinted8-scheme.parse(yaml) -> tinted8-scheme` - parse a Tinted8 YAML scheme
- `tinted8-scheme.metadata() -> tinted8-scheme-metadata` - extended metadata
- `tinted8-scheme.palette() -> list<palette-entry>` - full 33-color palette (11
  colors × 3 variants)
- `tinted8-scheme.syntax-color(key) -> color` - look up a syntax color by scope
  (e.g. `"keyword.control.import"`)
- `tinted8-scheme.ui-color(key) -> color` - look up a UI color by property
  (e.g. `"status.error"`)
- `tinted8-scheme.syntax-keys() -> list<string>` - list all valid syntax scope
  keys
- `tinted8-scheme.ui-keys() -> list<string>` - list all valid UI property keys
- `supported-builder-spec-version() -> string`
- `supported-styling-spec-version() -> string`

## Usage from other languages

Once built, the `.wasm` component can be used from any language with WASI
component support. Generate bindings for your language using `wit-bindgen` or
language-specific tooling.

### Rust (via `wasmtime`)

```rust
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

// Generate bindings from the WIT
wasmtime::component::bindgen!({
    path: "tinted-builder/wit/world.wit",
});

fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(
        &engine,
        "target/wasm32-wasip2/release/tinted_builder.wasm",
    )?;

    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = TintedBuilder::instantiate(&mut store, &component, &linker)?;

    // Parse a scheme
    let scheme = instance
        .tinted_theming_tinted_builder_schemes()
        .scheme()
        .call_parse(&mut store, &scheme_yaml)?;

    // Render a template
    let rendered = instance
        .tinted_theming_tinted_builder_renderer()
        .call_render(&mut store, scheme, &template_string)?;

    Ok(())
}
```

### Python (via `wasmtime-py`)

```python
import wasmtime
from wasmtime import Config, Engine, Store, Component, Linker

config = Config()
config.wasm_component_model = True
engine = Engine(config)
component = Component.from_file(engine, "tinted_builder.wasm")

# Use generated bindings (via componentize-py or wit-bindgen)
# to call scheme parsing and template rendering
```

### JavaScript/TypeScript (via `jco`)

```sh
# Generate JS bindings from the component
npx @bytecodealliance/jco transpile tinted_builder.wasm -o ./tinted-builder-js
```

```javascript
import { schemes, renderer } from './tinted-builder-js/tinted_builder.js';

const scheme = schemes.Scheme.parse(schemeYaml);
const rendered = renderer.render(scheme, templateString);
```
