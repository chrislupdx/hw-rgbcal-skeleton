#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
pub const LEVELS: u32 = 16;

// this just sets the mutex and returns the result of the mutex lock, does this even use or set it ior intact with the hardware in any awy yet
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}
//why would need to detect for get_rgb_levels,can't we just clobber, or would get be to check the upper and lower limit of the the color range? (will numbers be precoded for things)
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels); //was there an imlpemented setter for htis somewhere
}
//update

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    let board = Microbit::default();
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT; //is this the resoultion we want <its the highest res the have>
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,  // is this right
        [saadc::ChannelConfig::single_ended(board.p2)],
    );
    let knob = Knob::new(saadc).await; 
    //how are we supposed to be handling the incoming changes from the knob?

    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);
    //how /when do we catch and turn the knob: is it an await change in knob or something
    // ui has access to knob and board, but not expcicit access access to the led
    // how do the rgb and ui futures work together
    let (rgb_res, ui_res) = join::join(rgb.run(), ui.run()).await;  //is htis like my central loop or somethign

    panic!("fell off end of main loop");
}
