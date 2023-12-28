use scraper::{Html, Selector};

struct Stock {
    name: String,
    trading_price: f32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let stock_name = "ARAMIT".to_string();
    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        println!("No CLI URL provided, using default.");
        format!("https://dsebd.org/displayCompany.php?name={stock_name}").into()
    };
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("#company > tbody > tr:nth-child(1) > td:nth-child(2)").unwrap();
    let element_of_trading_price = document.select(&selector).nth(0);
    let binding = element_of_trading_price.unwrap().text().collect::<Vec<_>>();
    let trading_price = binding.first().unwrap().to_string().parse::<f32>().unwrap();
    let stock = Stock {
        name: stock_name,
        trading_price,
    };
    println!("{:?}", stock.trading_price);
    Ok(())
}
