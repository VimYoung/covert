#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use embassy_nrf::gpio::{Input, Level, Output, Pull};
use embassy_nrf::peripherals::*;
use embassy_nrf::usb::Driver as UsbDriver;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler;
});

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    PassiveJackUsb,
    BtSingle,
    BtMulti,
}

static mut MODE: Mode = Mode::PassiveJackUsb;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let usb_drv = UsbDriver::new(p.USBD, Irqs);
    spawner.spawn(usb_task(usb_drv)).unwrap();

    let btn = Input::new(p.P0_11, Pull::Up);
    let mut led = Output::new(p.P1_15, Level::Low);

    let mut bt = bluetooth::BtController::new();

    loop {
        match unsafe { MODE } {
            Mode::PassiveJackUsb => {
                audio::route_jack_to_usb();
                audio::route_usb_to_jack();
                led.set_low();
            }
            Mode::BtSingle => {
                bt.enable_single();
                audio::route_usb_or_jack_to_bt();
                led.set_high();
            }
            Mode::BtMulti => {
                bt.enable_multi();
                audio::route_usb_or_jack_to_bt_multi();
                led.toggle();
                Timer::after(Duration::from_millis(500)).await;
            }
        }

        if btn.is_low() {
            unsafe {
                MODE = match MODE {
                    Mode::PassiveJackUsb => Mode::BtSingle,
                    Mode::BtSingle => Mode::BtMulti,
                    Mode::BtMulti => Mode::PassiveJackUsb,
                };
            }
            Timer::after(Duration::from_millis(300)).await;
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}

#[embassy_executor::task]
async fn usb_task(mut driver: UsbDriver<'static, USBD>) {
    loop {
        driver.run().await;
    }
}

mod bluetooth {
    pub struct BtController;

    impl BtController {
        pub fn new() -> Self {
            Self
        }
        pub fn enable_single(&mut self) {}
        pub fn enable_multi(&mut self) {}
    }
}

mod audio {
    pub fn route_jack_to_usb() {}
    pub fn route_usb_to_jack() {}
    pub fn route_usb_or_jack_to_bt() {}
    pub fn route_usb_or_jack_to_bt_multi() {}
}
