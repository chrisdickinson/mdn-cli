use bat::{Input, PagingMode, PrettyPrinter};
use std::io::Write;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), surf::Error> {
    //→ curl -I 'https://api.duckduckgo.com/?format=json&q=%21%20site%3Adeveloper.mozilla.org%20accept-header'
    let args: Vec<String> = std::iter::once("! site:developer.mozilla.org".to_string())
        .chain(std::env::args().skip(1))
        .collect();

    if args.len() == 1 {
        eprintln!(
            r#"
mdn
Search the Mozilla Developer Network documentation for a given query
and display the top result as markdown in your terminal.

USAGE:
    mdn http accept header
    mdn queryselectorall
"#
        );
        std::process::exit(1);
    }

    let args = args.join(" ");
    let query = urlencoding::encode(args.as_str());

    let url = format!("https://api.duckduckgo.com/?q={}&format=json", query);
    let response = surf::get(url.as_str()).await?;
    let location = response
        .header("location")
        .map(|xs| xs.as_str().to_owned())
        .unwrap_or_else(Default::default);

    if location.is_empty() {
        async_std::task::sleep(Duration::from_millis(200)).await;

        std::io::stderr().write_all(b" No results.")?;
        std::process::exit(1);
    }

    let body = surf::get(location.as_str()).recv_string().await?;

    let document = scraper::Html::parse_document(body.as_str());
    let article_sel = scraper::Selector::parse("article.main-page-content").unwrap();

    let article = document
        .select(&article_sel)
        .next()
        .map(|xs| xs.inner_html())
        .unwrap_or_else(|| "".to_string());

    let html = format!("<article>{}</article>", article);

    let term_width = if let Some((w, _)) = term_size::dimensions() {
        w
    } else {
        120
    };

    let markdown = html2text::from_read(html.as_bytes(), term_width);
    PrettyPrinter::new()
        .pager("less")
        .paging_mode(PagingMode::QuitIfOneScreen)
        .header(true)
        .grid(true)
        .input(
            Input::from_bytes(markdown.as_bytes())
                .name("mdn.md")
                .title(url)
                .kind("Search Result"),
        )
        .print()
        .unwrap();

    Ok(())
}
