# mdn-cli

A command line tool for displaying the top DuckDuckGo search result for a MDN
query in your terminal. Automatically paginated.

```
$ mdn accept header

# Accept

The **`Accept`** request HTTP header advertises which content types, expressed as [MIME types][1], the client is able to understand. Using [content negotiation][2],
the server then selects one of the proposals, uses it and informs the client of its choice with the [`Content-Type`][3] response header. Browsers set adequate values
for this header depending on the context where the request is done: when fetching a CSS stylesheet a different value is set for the request than when fetching an
image, video or a script.

[...]
```

## Colophon

Results via DuckDuckGo's [Instant Answer] API.

[Instant Answer]: https://duckduckgo.com/api

Crates:

- [`html2text`](https://lib.rs/html2text)
- [`pager`](https://lib.rs/pager)
- [`scraper`](https://lib.rs/scraper)
- [`surf`](https://lib.rs/surf)
- [`term_size`](https://lib.rs/term_size)

## Installation

TODO

## License

MIT
