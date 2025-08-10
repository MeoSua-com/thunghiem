use std::time::Duration;

use clap::Parser;

use anyhow::Result;
use thirtyfour::prelude::*;
use tokio::time::sleep;

async fn click_ad(port: u16, wait_time: u64) -> Result<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--window-size=1920,1080")?; // Optional: set window size for visual consistency
    let driver = WebDriver::new(format!("http://localhost:{port}"), caps).await?;

    driver.goto("https://meosua.com/").await?;

    let ad = driver
        .query(By::Css("html > div:nth-child(4) > iframe"))
        .wait(Duration::from_secs(120), Duration::from_millis(100))
        .first()
        .await?;

    ad.enter_frame().await?;
    driver
        .find(By::Css("#container > div:nth-child(3) > button"))
        .await?
        .click()
        .await?;
    sleep(Duration::from_secs(wait_time)).await;

    driver.quit().await?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// WebDriver port
    #[arg(short, long, default_value_t = 9515)]
    port: u16,

    /// Time to wait in second before closing the browser
    #[arg(short, long, default_value_t = 5)]
    wait_time: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    click_ad(cli.port, cli.wait_time).await?;

    println!("Succeed");

    Ok(())
}
