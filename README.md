# `runwrap`

This is a library to wrap and unwrap Markdown, but for ease of editing and for
version control. It’s implemented in Rust.

`runwrap` is based on the `textwrap` and `pulldown_cmark` crates. The latter is
built to implement the CommonMark specification.

## Known defects

`runwrap` acts upon what `pulldown_cmark` identifies as paragraphs. This
includes normal paragraphs of running text as well as some list items etc.,
because it is tied to would-be `<p>` elements of HTML, not paragraphs in the
typographical sense.

```
This is a regular paragraph.

* This is a list item.

  Because this list item consists of multiple paragraphs, it can be affected by
  `runwrap`.
```

A list item made up only one paragraph of text is not identified as a paragraph
by `pulldown_cmark`, and is therefore ignored, even if it spans multiple lines.

```
* This is ignored because it’s one bullet for one paragraph.
* Also
  ignored.
```

This behaviour should be considered unstable. `runwrap` may develop to treat
such text the same as regular paragraphs.
