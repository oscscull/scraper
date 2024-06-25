use std::error::Error;
use csv::Writer;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let driver = WebDriver::new("http://localhost:9515", DesiredCapabilities::chrome()).await?;
    driver.goto("https://amazon.co.jp").await?;
    
    let elem_form = driver.find(By::Id("nav-search-bar-form")).await?;
    elem_form.find(By::Id("twotabsearchtextbox")).await?.send_keys("the brothers karamazov").await?;
    elem_form.find(By::Id("nav-search-submit-button")).await?.click().await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let search_results = driver.find_all(By::Css("div[data-component-type='s-search-result']")).await?;

    let mut wtr = Writer::from_path("search_results.csv")?;
    wtr.write_record(&["Title", "Author", "Price", "Rating"])?;

    for result in search_results {
        let title = get_text(&result, "h2 span").await;
        let author = get_text(&result, "div.a-row .a-size-base").await;
        let price = get_text(&result, "span.a-price-whole").await;
        let rating = get_attr(&result, "span[aria-label^='5つ星のうち']", "aria-label").await;

        if let Err(e) = wtr.write_record(&[&title, &author, &price, &rating]) {
            eprintln!("Failed to write record to CSV: {}", e);
        }
    }

    wtr.flush()?;
    driver.quit().await?;
    Ok(())
}

async fn get_text(element: &WebElement, selector: &str) -> String {
    match element.find(By::Css(selector)).await {
        Ok(el) => el.text().await.unwrap_or_default(),
        Err(_) => String::new(),
    }
}

async fn get_attr(element: &WebElement, selector: &str, attr: &str) -> String {
    match element.find(By::Css(selector)).await {
        Ok(el) => el.attr(attr).await.unwrap_or_default().unwrap_or_default(),
        Err(_) => String::new(),
    }
}