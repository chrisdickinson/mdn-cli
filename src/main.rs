use std::io::Write;

#[async_std::main]
async fn main() -> Result<(), surf::Error> {
    //→ curl -I 'https://api.duckduckgo.com/?format=json&q=%21%20site%3Adeveloper.mozilla.org%20accept-header'
    let args: Vec<String> = std::iter::once("! site:developer.mozilla.org".to_string())
        .chain(std::env::args().skip(1)).collect();
    let args = args.join(" ");
    let query = urlencoding::encode(args.as_str());

    if false {
    async_std::task::spawn(async {
        for chr in "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏".chars().cycle() {
            write!(std::io::stdout(), "\r {} (fetching results from DuckDuckGo...)\r", chr);
            async_std::task::sleep(std::time::Duration::from_millis(166));
        }
    });
    }

    let url = format!("https://api.duckduckgo.com/?q={}&format=json", query);
    let mut response = surf::get(url.as_str()).await?;
    let location = response.header("location")
        .map(|xs| xs.as_str().to_owned())
        .unwrap_or_else(Default::default);

    eprintln!("res={}", response.body_string().await?);
    if location.is_empty() {
        std::io::stdout().write_all(b"\r")?;
        std::io::stdout().write_all(b"No results.");
        return Ok(())
    }

    let body = surf::get(location.as_str()).recv_string().await?;

    let document = scraper::Html::parse_document(body.as_str()); 
    let title_h1_sel = scraper::Selector::parse("header h1.title").unwrap();
    let article_sel = scraper::Selector::parse("article#wikiArticle").unwrap();

    let title = document.select(&title_h1_sel).next().map(|xs| {
        xs.inner_html()
    }).unwrap_or_else(|| "".to_string());

    let article = document.select(&article_sel).next().map(|xs| {
        xs.inner_html()
    }).unwrap_or_else(|| "".to_string());

    let html = format!("<h1>{}</h1><article>{}</article>", title, article);

    let term_width = if let Some((w, _)) = term_size::dimensions() {
        w
    } else {
        120
    };

    std::io::stdout().write_all(b"\r")?;
    std::io::stdout().write_all(html2text::from_read(html.as_bytes(), term_width).as_bytes())?;

    Ok(())
}
