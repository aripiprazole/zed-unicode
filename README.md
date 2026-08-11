# zed-unicode

Add completion to unicode characters in Zed. It completes by default the mathematical symbols, the arrows, the greek letters.

It's very simple to use, you can just type `->` which transforms into `→` and `->>` into `↠`.

## Settings

Configure the language server in your `settings.json`:

```json
{
  "lsp": {
    "unicode": {
      "settings": {
        "include_all_symbols": false,
        "prefix": ""
      }
    }
  }
}
```

- `include_all_symbols` — when `true`, pulls in a trigger for (almost) every named
  Unicode character, not just the curated math/arrow/Greek set.
- `prefix` — require this string before a snippet's trigger word to complete, e.g.
  `"u"` means you need to type `ulambda` instead of bare `lambda`. Empty (the
  default) requires no prefix, matching the classic behavior.

  Use an **alphanumeric** prefix, not punctuation. Zed's own completion popup
  computes its filter query using only "word" characters immediately before the
  cursor (independent of the language server), so a punctuation prefix like `\`
  gets excluded from what Zed searches for even though the language server
  handles it correctly — the popup just won't show the match. A letter or digit
  prefix doesn't have this problem.

## Pointing at a local build (development)

To test a locally built `unicode-ls` instead of the one downloaded from GitHub
releases, use Zed's built-in binary override — do **not** need any custom setting
for this:

```json
{
  "lsp": {
    "unicode": {
      "binary": {
        "path": "/absolute/path/to/unicode-ls",
        "arguments": ["--prefix", "u"]
      }
    }
  }
}
```

Note: once you set `binary.path`, Zed builds the launch command directly from
`binary.path`/`binary.arguments` and does not consult `settings.include_all_symbols`
or `settings.prefix` at all — pass any flags you need explicitly via `arguments`
in that case.
