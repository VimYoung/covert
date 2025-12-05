#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use nrf52840_hal as hal;

use hal::{
    pac,
    gpio::{p0::Parts as P0Parts, Level, Output, PushPull, Input, PullUp},
    prelude::*,
    spim::Spim,
    usbd::{UsbPeripheral, Usbd},
    timer::Timer,
};

#[derive(Clone, Copy)]
enum DeviceMode {
    PassiveJackUsb,
    BtSingle,
    BtMulti,
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let p0 = P0Parts::new(p.P0);

    // --- Button ---
    let button = p0.p0_11.into_pullup_input();

    // --- RGB LED ---
    let mut led_r = p0.p0_12.into_push_pull_output(Level::Low);
    let mut led_g = p0.p0_13.into_push_pull_output(Level::Low);
    let mut led_b = p0.p0_14.into_push_pull_output(Level::Low);

    // --- Audio input/output pins ---
    let _audio_in = p0.p0_02.into_floating_input();
    let _audio_out = p0.p0_03.into_push_pull_output(Level::Low);

    // --- USB ---
    let usb_periph = UsbPeripheral::new(p.USBD, p0.p0_25, p0.p0_26);
    let _usbd = Usbd::new(usb_periph);

    // --- SPI / I2C for BT module ---
    let spim = Spim::new(
        p.SPIM0,
        hal::spim::Pins {
            sck: p0.p0_15.into_push_pull_output(Level::Low).degrade(),
            mosi: Some(p0.p0_16.into_push_pull_output(Level::Low).degrade()),
            miso: Some(p0.p0_17.into_floating_input().degrade()),
        },
        hal::spim::Frequency::M8,
        hal::spim::MODE_0,
        0,
    );

    let mut mode = DeviceMode::PassiveJackUsb;
    let mut last_button_state = true;

    loop {
        let button_state = button.is_high().unwrap_or(true);

        if last_button_state && !button_state {
            mode = match mode {
                DeviceMode::PassiveJackUsb => DeviceMode::BtSingle,
                DeviceMode::BtSingle => DeviceMode::BtMulti,
                DeviceMode::BtMulti => DeviceMode::PassiveJackUsb,
            };
        }
        last_button_state = button_state;

        match mode {
            DeviceMode::PassiveJackUsb => {
                led_r.set_high();
                led_g.set_low();
                led_b.set_low();
                audio::route_jack_to_usb();
                audio::route_usb_to_jack();
            }
            DeviceMode::BtSingle => {
                led_r.set_low();
                led_g.set_high();
                led_b.set_low();
                bluetooth::enable_single(&spim);
                audio::route_usb_or_jack_to_bt();
            }
            DeviceMode::BtMulti => {
                led_r.set_low();
                led_g.set_low();
                led_b.set_high();
                bluetooth::enable_multi(&spim);
                audio::route_usb_or_jack_to_bt_multi();
            }
        }

        Timer::after(hal::time::Hertz(100)).unwrap(); // simple delay
    }
}

mod bluetooth {
    use embedded_hal::blocking::spi::Write;

    pub fn enable_single<SPI: Write<u8>>(_spi: &SPI) {}
    pub fn enable_multi<SPI: Write<u8>>(_spi: &SPI) {}
}

mod audio {
    pub fn route_jack_to_usb() {}
    pub fn route_usb_to_jack() {}
    pub fn route_usb_or_jack_to_bt() {}
    pub fn route_usb_or_jack_to_bt_multi() {}
}
