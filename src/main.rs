use std::{env, time::Duration};

use anyhow::Result;
use chromiumoxide::{
    Page,
    browser::{Browser, BrowserConfig},
    layout::Point,
};
use futures::StreamExt;
use tokio::time::sleep;

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;
const AD_X: f64 = 506.;
const AD_Y: f64 = 27.;
const AD_COUNT: usize = 4;
const WAIT_TIME: Duration = Duration::from_secs(3);

async fn click(page: &Page, browser: &Browser) -> Result<()> {
    let point = Point::new(AD_X, AD_Y);
    let mut current_tab_count = browser.pages().await?.len();
    let targeted_tab_count = current_tab_count + AD_COUNT;

    while current_tab_count < targeted_tab_count {
        let mut new_tab_count = current_tab_count;

        while new_tab_count == current_tab_count {
            page.click(point).await?;
            new_tab_count = browser.pages().await?.len();
            sleep(Duration::from_millis(100)).await;
        }
        current_tab_count = new_tab_count;

        println!("Task left: {}", targeted_tab_count - current_tab_count);
        sleep(WAIT_TIME).await;

        page.bring_to_front().await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .window_size(WIDTH, HEIGHT)
            .no_sandbox()
            .build()
            .unwrap(),
    )
    .await?;
    tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await;
        }
    });

    let url = env::var("URL").unwrap();

    let page = browser.new_page("about:blank").await?;
    page.enable_stealth_mode().await?;
    page.goto(url).await?;

    click(&page, &browser).await?;

    browser.close().await?;

    Ok(())
}
