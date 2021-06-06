This is a library to wrap and unwrap Markdown, both for ease of editing and for
version control. It’s implemented in functional-style Rust.

`runwrap` is based on the `textwrap` and `pulldown_cmark` crates. The latter is
designed to implement the CommonMark specification.

## Goals

* Wrapped text suitable for editing and version control.
* Unwrapped text suitable for searching and complex markup resolution.
* Easy integration with higher-level languages and editors.
* Idempotence.
* Reasonable performance.

## Bindings in other languages

* Python: `punwrap` ([repo](https://github.com/veikman/punwrap)).

## Known defects

The following may or may not be fixed and should therefore be considered
unstable behaviour.

### No advanced options

Currently, only a maximum width can be specified for wrapping text. More
advanced options cannot currently be passed through `runwrap` to `textwrap`.

### Width is applied locally to each paragraph

Width settings passed to `wrap` and `rewrap` are applied in the scope of each
individual paragraph. This means that the overall line width of list items can
exceed the passed value.

For the same reason, list items wrap to column zero; they are not neatly
indented.

### Single-paragraph list items are ignored

`runwrap` acts only upon what `pulldown_cmark` identifies as paragraphs. This
includes normal paragraphs of running text as well as some list items etc.,
because it is tied to would-be `<p>` elements of HTML, not paragraphs in the
typographical sense.

```
This is a regular paragraph.

* This is a list item.

  Because this list item consists of multiple paragraphs, it
  (each of its two paragraphs) can be affected by `runwrap`.
```

A list item made up of one paragraph of text is not identified as a paragraph
by `pulldown_cmark`, and is therefore ignored, even if it spans multiple lines.

```
* This is ignored because it’s one bullet for one paragraph.
* Also
  ignored.
```

## Legal

Copyright 2021 Viktor Eikman

`runwrap` is licensed as detailed in the accompanying file LICENSE.
