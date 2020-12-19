# `yew-md`

`yew-md` is a Yew library for markdown parsing. It uses

- [`pulldown-cmark`](https://github.com/raphlinus/pulldown-cmark/) for markdown parsing to HTML
- [`syntect`]() for syntax highlighing (support coming in future)

## Installation

To add yew-md to your project, include it in your `Cargo.toml` file

```toml
[dependencies]
yew-md = "0.1"
```

### Features

- `syntax-highlighting`  
  Enables syntax highlighting using `syntect`

## Usage

You can use the `markdown` component to parse markdown from your html.

### Example

```rust
let input = "
# Hello world

This is a ~~complicated~~ *very simple* example.
";

html! {
    <Markdown content=input />
}
```

## Configuration

### Parser options

An `Options` object can be passed to `options` prop to customize the parser's options. By
default, `ENABLE_STRIKETHROUGH` option is enabled.

```rust
html! { <Markdown content=input options=Options::all() /> }
```

### Syntax Highlighting

Coming soon:tm:
