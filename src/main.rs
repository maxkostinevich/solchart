/*

 ███╗   ███╗ █████╗ ██╗  ██╗██╗ ██████╗ ██████╗    ██████╗ ███████╗██╗   ██╗
 ████╗ ████║██╔══██╗╚██╗██╔╝██║██╔════╝██╔═══██╗   ██╔══██╗██╔════╝██║   ██║
 ██╔████╔██║███████║ ╚███╔╝ ██║██║     ██║   ██║   ██║  ██║█████╗  ██║   ██║
 ██║╚██╔╝██║██╔══██║ ██╔██╗ ██║██║     ██║   ██║   ██║  ██║██╔══╝  ╚██╗ ██╔╝
 ██║ ╚═╝ ██║██║  ██║██╔╝ ██╗██║╚██████╗╚██████╔╝██╗██████╔╝███████╗ ╚████╔╝
 ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═════╝ ╚═╝╚═════╝ ╚══════╝  ╚═══╝


 -------------------------------------------------------------------------------
 (c) Max Kostinevich / https://maxico.dev
 -------------------------------------------------------------------------------

*/

use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

use cli_candlestick_chart::{Candle, Chart};

#[derive(Debug, Clone, Deserialize)]
struct BinanceKlinesItem {
    open_time: u64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    close_time: u64,
    quote_asset_volume: String,
    number_of_trades: u64,
    taker_buy_base_asset_volume: String,
    taker_buy_quote_asset_volume: String,
    ignore: String,
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let candles = reqwest::blocking::get(
            "https://api.binance.com/api/v1/klines?symbol=SOLUSDT&interval=1s",
        )?
        .json::<Vec<BinanceKlinesItem>>()?
        .iter()
        .map(|candle| {
            Candle::new(
                candle.open.parse::<f64>().unwrap(),
                candle.high.parse::<f64>().unwrap(),
                candle.low.parse::<f64>().unwrap(),
                candle.close.parse::<f64>().unwrap(),
                Some(candle.volume.parse::<f64>().unwrap()),
                Some(candle.open_time as i64),
            )
        })
        .collect::<Vec<Candle>>();

        let mut chart = Chart::new(&candles);

        chart.set_name(String::from("SOL/USDT"));
        chart.set_bull_color(34, 197, 94);
        chart.set_bear_color(236, 72, 153);
        chart.set_vol_bull_color(34, 197, 94);
        chart.set_vol_bear_color(236, 72, 153);
        chart.set_volume_pane_height(2);
        chart.set_volume_pane_enabled(false);
        //  chart.set_volume_pane_unicode_fill('*');

        clear_screen(); // Clear the screen before drawing the chart

        chart.draw();

        // Sleep for one minute before fetching and drawing the chart again
        thread::sleep(Duration::from_secs(5));
    }

    // Ok(())
}
