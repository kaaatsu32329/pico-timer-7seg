use bsp::hal::gpio::{Output, Pin, PinId, PushPull};
use cortex_m::delay::Delay;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use rp_pico as bsp;

pub struct Digit<I1, I2, I3, I4>
where
    I1: PinId,
    I2: PinId,
    I3: PinId,
    I4: PinId,
{
    one: Pin<I1, Output<PushPull>>,
    two: Pin<I2, Output<PushPull>>,
    three: Pin<I3, Output<PushPull>>,
    four: Pin<I4, Output<PushPull>>,
}

impl<I1, I2, I3, I4> Digit<I1, I2, I3, I4>
where
    I1: PinId,
    I2: PinId,
    I3: PinId,
    I4: PinId,
{
    pub fn new(
        one: Pin<I1, Output<PushPull>>,
        two: Pin<I2, Output<PushPull>>,
        three: Pin<I3, Output<PushPull>>,
        four: Pin<I4, Output<PushPull>>,
    ) -> Self {
        Self {
            one,
            two,
            three,
            four,
        }
    }
}

pub struct Segment<I1, I2, I3, I4, I5, I6, I7, I8>
where
    I1: PinId,
    I2: PinId,
    I3: PinId,
    I4: PinId,
    I5: PinId,
    I6: PinId,
    I7: PinId,
    I8: PinId,
{
    a: Pin<I1, Output<PushPull>>,
    b: Pin<I2, Output<PushPull>>,
    c: Pin<I3, Output<PushPull>>,
    d: Pin<I4, Output<PushPull>>,
    e: Pin<I5, Output<PushPull>>,
    f: Pin<I6, Output<PushPull>>,
    g: Pin<I7, Output<PushPull>>,
    dp: Pin<I8, Output<PushPull>>,
}

impl<I1, I2, I3, I4, I5, I6, I7, I8> Segment<I1, I2, I3, I4, I5, I6, I7, I8>
where
    I1: PinId,
    I2: PinId,
    I3: PinId,
    I4: PinId,
    I5: PinId,
    I6: PinId,
    I7: PinId,
    I8: PinId,
{
    pub fn new(
        a: Pin<I1, Output<PushPull>>,
        b: Pin<I2, Output<PushPull>>,
        c: Pin<I3, Output<PushPull>>,
        d: Pin<I4, Output<PushPull>>,
        e: Pin<I5, Output<PushPull>>,
        f: Pin<I6, Output<PushPull>>,
        g: Pin<I7, Output<PushPull>>,
        dp: Pin<I8, Output<PushPull>>,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            dp,
        }
    }

    pub fn display_single_number(&mut self, number: u8) {
        match number {
            0 => {
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_high().unwrap();
                self.g.set_low().unwrap();
                self.dp.set_low().unwrap();
            }
            1 => {
                self.a.set_low().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_low().unwrap();
                self.e.set_low().unwrap();
                self.f.set_low().unwrap();
                self.g.set_low().unwrap();
                self.dp.set_low().unwrap();
            }
            2 => {
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_low().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_low().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            3 => {
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_low().unwrap();
                self.f.set_low().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            4 => {
                self.a.set_low().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_low().unwrap();
                self.e.set_low().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            5 => {
                self.a.set_high().unwrap();
                self.b.set_low().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_low().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            6 => {
                self.a.set_high().unwrap();
                self.b.set_low().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            7 => {
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_low().unwrap();
                self.e.set_low().unwrap();
                self.f.set_low().unwrap();
                self.g.set_low().unwrap();
                self.dp.set_low().unwrap();
            }
            8 => {
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_high().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            9 => {
                self.a.set_high().unwrap();
                self.b.set_high().unwrap();
                self.c.set_high().unwrap();
                self.d.set_high().unwrap();
                self.e.set_low().unwrap();
                self.f.set_high().unwrap();
                self.g.set_high().unwrap();
                self.dp.set_low().unwrap();
            }
            _ => {
                self.a.set_low().unwrap();
                self.b.set_low().unwrap();
                self.c.set_low().unwrap();
                self.d.set_low().unwrap();
                self.e.set_low().unwrap();
                self.f.set_low().unwrap();
                self.g.set_low().unwrap();
                self.dp.set_low().unwrap();
            }
        }
    }
}

pub struct Display<SI1, SI2, SI3, SI4, SI5, SI6, SI7, SI8, DI1, DI2, DI3, DI4>
where
    SI1: PinId,
    SI2: PinId,
    SI3: PinId,
    SI4: PinId,
    SI5: PinId,
    SI6: PinId,
    SI7: PinId,
    SI8: PinId,
    DI1: PinId,
    DI2: PinId,
    DI3: PinId,
    DI4: PinId,
{
    segment: Segment<SI1, SI2, SI3, SI4, SI5, SI6, SI7, SI8>,
    digit: Digit<DI1, DI2, DI3, DI4>,
}

impl<SI1, SI2, SI3, SI4, SI5, SI6, SI7, SI8, DI1, DI2, DI3, DI4>
    Display<SI1, SI2, SI3, SI4, SI5, SI6, SI7, SI8, DI1, DI2, DI3, DI4>
where
    SI1: PinId,
    SI2: PinId,
    SI3: PinId,
    SI4: PinId,
    SI5: PinId,
    SI6: PinId,
    SI7: PinId,
    SI8: PinId,
    DI1: PinId,
    DI2: PinId,
    DI3: PinId,
    DI4: PinId,
{
    pub fn new(
        segment: Segment<SI1, SI2, SI3, SI4, SI5, SI6, SI7, SI8>,
        digit: Digit<DI1, DI2, DI3, DI4>,
    ) -> Self {
        Self { segment, digit }
    }

    pub fn num_display(&mut self, number: i32, delay: &mut Delay) {
        if number > 9999 || number < -999 {
            // Out of range
            todo!();
        } else if number.is_negative() {
            // Display `-` in digit 4.
            todo!();
        } else {
            let digit4 = (number / 1000) as u8; // ToDo: if 0 -> do not display
            let digit3 = (number / 100 % 10) as u8;
            let digit2 = (number / 10 % 10) as u8;
            let digit1 = (number % 10) as u8;

            self.digit.one.set_low().unwrap();
            self.digit.two.set_high().unwrap();
            self.digit.three.set_high().unwrap();
            self.digit.four.set_high().unwrap();
            self.segment.display_single_number(digit1);
            delay.delay_ms(1);

            self.digit.one.set_high().unwrap();
            self.digit.two.set_low().unwrap();
            self.digit.three.set_high().unwrap();
            self.digit.four.set_high().unwrap();
            self.segment.display_single_number(digit2);
            delay.delay_ms(1);

            self.digit.one.set_high().unwrap();
            self.digit.two.set_high().unwrap();
            self.digit.three.set_low().unwrap();
            self.digit.four.set_high().unwrap();
            self.segment.display_single_number(digit3);
            delay.delay_ms(1);

            self.digit.one.set_high().unwrap();
            self.digit.two.set_high().unwrap();
            self.digit.three.set_high().unwrap();
            self.digit.four.set_low().unwrap();
            self.segment.display_single_number(digit4);
            delay.delay_ms(1);
        }
    }
}
