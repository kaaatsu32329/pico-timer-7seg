#![no_std]
#![no_main]

mod segment_display;

use bsp::entry;
use core::{cell::RefCell, ops::DerefMut};
use critical_section::Mutex;
use defmt_rtt as _;
use fugit::MicrosDurationU32;
use panic_probe as _;
use rp2040_hal as hal;
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use hal::{pac::interrupt, timer::Alarm};
use segment_display::*;

const SCAN_TIME: MicrosDurationU32 = MicrosDurationU32::secs(1_000_000);
static COUNTER: Mutex<RefCell<Option<i32>>> = Mutex::new(RefCell::new(None));
static ALARM: Mutex<RefCell<Option<bsp::hal::timer::Alarm0>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<bsp::hal::timer::Timer>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = bsp::hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let mut alarm = timer.alarm_0().unwrap();
    alarm.schedule(SCAN_TIME).unwrap();
    alarm.enable_interrupt();

    critical_section::with(|cs| {
        // cortex_m::interrupt::free(|cs| {
        COUNTER.borrow(cs).replace(Some(999));
        TIMER.borrow(cs).replace(Some(timer));
        ALARM.borrow(cs).replace(Some(alarm));
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_0);
    }

    // Set display configlation.
    let mut display = Display::new(
        Segment::new(
            pins.gpio8.into_push_pull_output(),
            pins.gpio6.into_push_pull_output(),
            pins.gpio10.into_push_pull_output(),
            pins.gpio12.into_push_pull_output(),
            pins.gpio13.into_push_pull_output(),
            pins.gpio7.into_push_pull_output(),
            pins.gpio9.into_push_pull_output(),
            pins.gpio11.into_push_pull_output(),
        ),
        Digit::new(
            pins.gpio2.into_push_pull_output(),
            pins.gpio3.into_push_pull_output(),
            pins.gpio4.into_push_pull_output(),
            pins.gpio5.into_push_pull_output(),
        ),
    );

    // Main loop
    loop {
        // cortex_m::asm::wfi();
        critical_section::with(|cs| {
            display.num_display(COUNTER.borrow(cs).take().unwrap(), &mut delay);
        });
    }
}

#[allow(non_snake_case)]
#[interrupt]
fn TIMER_IRQ_0() {
    critical_section::with(|cs| {
        if let Some(ref mut alarm) = ALARM.borrow(cs).borrow_mut().deref_mut() {
            alarm.schedule(SCAN_TIME).unwrap();
            alarm.clear_interrupt();
        }

        if let Some(ref mut counter) = COUNTER.borrow(cs).borrow_mut().deref_mut() {
            let update = *counter + 1;
            COUNTER.borrow(cs).replace_with(|_| Some(update));
        }
    });
}
