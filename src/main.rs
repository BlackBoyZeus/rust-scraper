use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use csv::Writer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = vec![
        "https://docs.gtk.org/gobject/concepts.html",
        "https://gstreamer.freedesktop.org/documentation/deploying/index.html?gi-language=c",
        "https://gstreamer.freedesktop.org/documentation/?gi-language=c",
        "https://gstreamer.freedesktop.org/documentation/application-development/basics/pads.html?gi-language=c",
        "https://gstreamer.freedesktop.org/documentation/application-development/basics/elements.html?gi-language=c",
        "https://gstreamer.freedesktop.org/documentation/gstreamer/gstplugin.html?gi-language=c",
        "https://gstreamer.freedesktop.org/documentation/gstreamer/gi-index.html?gi-language=c",
    ];

    let mut writer = Writer::from_path("documentation.csv")?;
    writer.write_record(&["URL", "Title", "Content"])?;

    let client = Client::new();

    for url in urls {
        let response = client.get(url).send().await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let title_selector = Selector::parse("h1").unwrap();
        let content_selectors = vec![
            Selector::parse("p").unwrap(),
            Selector::parse("pre").unwrap(),
            Selector::parse("code").unwrap(),
            Selector::parse("li").unwrap(),
            Selector::parse("table").unwrap(),
        ];

        let title = document
            .select(&title_selector)
            .next()
            .map(|element| element.inner_html())
            .unwrap_or_else(|| "No title found".to_string());

        let mut content = String::new();
        for selector in &content_selectors {
            for element in document.select(selector) {
                content.push_str(&element.inner_html());
                content.push('\n');
            }
        }

        writer.write_record(&[url, title.as_str(), content.as_str()])?;
    }

    writer.flush()?;
    Ok(())
}