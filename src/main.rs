#![no_std]
#![no_main]

mod segment_display;

use bsp::entry;
use core::cell::RefCell;
use critical_section::Mutex;
use defmt_rtt as _;
use fugit::MicrosDurationU32;
use panic_probe as _;
use rp2040_hal as rhal;
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use rhal::{
    pac::interrupt,
    timer::{Alarm, Alarm0},
};
use segment_display::*;

type InterruptData = (u32, Alarm0);

const SCAN_TIME: MicrosDurationU32 = MicrosDurationU32::secs(1);
static COUNTER: Mutex<RefCell<Option<InterruptData>>> = Mutex::new(RefCell::new(None));

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

    critical_section::with(|cs| {
        let mut alarm = timer.alarm_0().unwrap();
        let _ = alarm.schedule(SCAN_TIME);
        alarm.enable_interrupt();
        COUNTER.borrow(cs).replace(Some((0, alarm)));
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
            display.num_display(COUNTER.borrow(cs).take().unwrap().0 as i32, &mut delay);
        });
    }
}

#[interrupt]
fn TIMER_IRQ_0() {
    critical_section::with(|cs| {
        let count_alarm = COUNTER.borrow(cs).take();
        if let Some((mut count, mut alarm)) = count_alarm {
            alarm.clear_interrupt();
            let _ = alarm.schedule(SCAN_TIME);
            count += 1;
            COUNTER
                .borrow(cs)
                .replace_with(|_| Some((count, alarm)));
        }
    });
}
