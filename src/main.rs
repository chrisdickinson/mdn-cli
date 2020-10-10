use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), surf::Error> {
    //→ curl -I 'https://api.duckduckgo.com/?format=json&q=%21%20site%3Adeveloper.mozilla.org%20accept-header'
    let args: Vec<String> = std::iter::once("! site:developer.mozilla.org".to_string())
        .chain(std::env::args().skip(1))
        .collect();
    let args = args.join(" ");
    let query = urlencoding::encode(args.as_str());

    let loading = Arc::new(AtomicBool::new(true));
    let loading_clone = Arc::clone(&loading);
    async_std::task::spawn(async move {
        for chr in "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏".chars().cycle() {
            let mut stdout = std::io::stdout();
            write!(stdout, "\r {} Fetching results from DuckDuckGo...", chr).ok();
            stdout.flush().ok();

            async_std::task::sleep(Duration::from_millis(32)).await;
            if !loading_clone.load(Ordering::Relaxed) {
                break;
            }
        }
    });

    let url = format!("https://api.duckduckgo.com/?q={}&format=json", query);
    let response = surf::get(url.as_str()).await?;
    let location = response
        .header("location")
        .map(|xs| xs.as_str().to_owned())
        .unwrap_or_else(Default::default);

    if location.is_empty() {
        std::io::stdout().write_all(b"\rNo results.")?;
        return Ok(());
    }

    let body = surf::get(location.as_str()).recv_string().await?;

    let document = scraper::Html::parse_document(body.as_str());
    let title_h1_sel = scraper::Selector::parse("header h1.title").unwrap();
    let article_sel = scraper::Selector::parse("article#wikiArticle").unwrap();

    let title = document
        .select(&title_h1_sel)
        .next()
        .map(|xs| xs.inner_html())
        .unwrap_or_else(|| "".to_string());

    let article = document
        .select(&article_sel)
        .next()
        .map(|xs| xs.inner_html())
        .unwrap_or_else(|| "".to_string());

    let html = format!("<h1>{}</h1><article>{}</article>", title, article);

    let term_width = if let Some((w, _)) = term_size::dimensions() {
        w
    } else {
        120
    };

    loading.swap(false, Ordering::Relaxed);
    async_std::task::sleep(Duration::from_millis(200)).await;

    pager::Pager::with_pager("less -r").setup();
    std::io::stdout().write_all(b"\x1b[4m(Results from DuckDuckGo)\x1b[0m\n\n")?;
    std::io::stdout().write_all(html2text::from_read(html.as_bytes(), term_width).as_bytes())?;

    Ok(())
}
