//! This example shows powerful PIO module in the RP2040 chip to communicate with WS2812 LED modules.
//! See (https://www.sparkfun.com/categories/tags/ws2812)

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::{clk_sys_freq, RoscRng};
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Instant, Ticker, Timer};
use fixed::types::U24F8;
use log::info;
use smart_led_pio_sr::{PioWs2812SR, PioWs2812SRProgram};
use smart_leds::RGB8;
use tetris::random::RandomGenerator;
use tetris::rotate::SuperRotationSystem;
use tetris::{CurrentPiece, Game};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

// 20092/500 = 40
// 5760/144 = 40
//  40 us / led
const NUM_LEDS: usize = 144;

enum PinResult {
    RisingEdge,
    FallingEdge,
    On,
    Off,
}

const DEBOUNCE: Duration = Duration::from_millis(30);

struct Button<'d> {
    input: Input<'d>,
    last_update: Instant,
    last_state: bool
}

impl<'d> Button<'d> {
    pub fn new(pin: Input<'d>) -> Self {
        Self { input: pin, last_update: Instant::now(), last_state: false }
    }

    pub fn get_state(&mut self) -> PinResult {
        let s = self.input.is_low();
        if s != self.last_state && self.last_update.elapsed() > DEBOUNCE {
            self.last_state = s;
            self.last_update = Instant::now();
            if s {
                PinResult::RisingEdge
            } else {
                PinResult::FallingEdge
            }
        } else {
            if self.last_state {
                PinResult::On
            } else {
                PinResult::Off
            }
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    let mut data = [
        [RGB8::default(); NUM_LEDS],
        [RGB8::default(); NUM_LEDS],
        [RGB8::default(); NUM_LEDS],
        [RGB8::default(); NUM_LEDS],
    ];

    // Common neopixel pins:
    // Thing plus: 8
    // Adafruit Feather: 16;  Adafruit Feather+RFM95: 4
    let program = PioWs2812SRProgram::new(&mut common);
    let mut ws2812 = PioWs2812SR::new(
        &mut common,
        sm0,
        p.DMA_CH0,
        p.PIN_0,
        p.PIN_1,
        p.PIN_2,
        &program,
    );

    let rot = SuperRotationSystem{};
    let rng = RandomGenerator::new(RoscRng);
    let mut game = Game::new(rng, rot);

    let mut left_pin = Button::new(Input::new(p.PIN_15, embassy_rp::gpio::Pull::Up));
    let mut soft_drop_pin = Button::new(Input::new(p.PIN_14, embassy_rp::gpio::Pull::Up));
    let mut right_pin = Button::new(Input::new(p.PIN_13, embassy_rp::gpio::Pull::Up));
    
    let mut hold_pin = Button::new(Input::new(p.PIN_12, embassy_rp::gpio::Pull::Up));
    let mut rotate_left_pin = Button::new(Input::new(p.PIN_11, embassy_rp::gpio::Pull::Up));
    let mut rotate_right_pin = Button::new(Input::new(p.PIN_10, embassy_rp::gpio::Pull::Up));
    let mut drop_pin = Button::new(Input::new(p.PIN_9, embassy_rp::gpio::Pull::Up));

    // Loop forever making RGB  values and pushing them out to the WS2812.
    for i in 0..4 {
        for l in 0..NUM_LEDS {
            data[i][l] = RGB8::new(64, 64, 64);
        }
    }
    let mut ticker = Ticker::every(Duration::from_millis(16));
    loop {
        for x in 0..10 {
            for y in 0..20 {
                draw_pixel(&mut data, x + 7, y + 2, game.board()[y as usize][x as usize]);
            }
        }

        let p = game.ghost_piece();
        let mut c = p.color();
        c.0 /= 2;
        c.1 /= 2;
        c.2 /= 2;
        draw_mask(&mut data, 22, 7, p.y() + 2, p.mask(), c);

        let p = game.current_piece();
        draw_mask(&mut data, 22, 7, p.y() + 2, p.mask(), p.color());

        for x in 0..4 {
            for y in 0..4 {
                draw_pixel(&mut data, x + 1, 18 + y, (0, 0, 0));
            }
        }

        if let Some(held) = game.held_piece() {
            let p = CurrentPiece::new(held, 0, 0, tetris::Rotation::Rotate0);
            draw_mask(&mut data, 24, 1, 18, p.mask(), p.color());
        }

        for (i, piece) in game.next_pieces().iter().enumerate() {
            let p = CurrentPiece::new(*piece, 0, 0, tetris::Rotation::Rotate0);
            for x in 0..4 {
                for y in 0..2 {
                    draw_pixel(&mut data, 19 + x, 20 - (3 * i as u32) + y, (0, 0, 0));
                }
            }
            draw_mask(&mut data, 24, 19, 20 - (3 * i as u32), p.mask(), p.color());
        }


        match left_pin.get_state() {
            PinResult::RisingEdge => game.set_left(true),
            PinResult::FallingEdge => game.set_left(false),
            _ => {},
        }

        match right_pin.get_state() {
            PinResult::RisingEdge => game.set_right(true),
            PinResult::FallingEdge => game.set_right(false),
            _ => {},
        }

        match soft_drop_pin.get_state() {
            PinResult::RisingEdge => game.set_drop(true),
            PinResult::FallingEdge => game.set_drop(false),
            _ => {},
        }

        match hold_pin.get_state() {
            PinResult::RisingEdge => game.hold(),
            _ => {},
        }

        match rotate_left_pin.get_state() {
            PinResult::RisingEdge => game.rotate_left(),
            _ => {},
        }

        match rotate_right_pin.get_state() {
            PinResult::RisingEdge => game.rotate_right(),
            _ => {},
        }

        match drop_pin.get_state() {
            PinResult::RisingEdge => game.hard_drop(),
            _ => {},
        }

        ws2812.write(&data).await;
        game.update();
        ticker.next().await;
    }
}


fn draw_pixel(frame: &mut [[RGB8; NUM_LEDS]; 4], x: u32, y: u32, color: (u8, u8, u8)) {
    let y = 24 - y - 1;
    let i = y/6;
    let x_off = if y % 2 == 0 {
        x
    } else {
        23-x
    };

    let l = (24 * (y % 6)) + x_off;
    frame[i as usize][l as usize] = RGB8::new(color.0, color.1, color.2)
}

fn draw_mask(frame: &mut [[RGB8; NUM_LEDS]; 4], draw_limit: u32, x_offset: u32, y: u32, mask: [u16; 4], color: (u8, u8, u8)) {
    for (i, m) in mask.iter().enumerate() {
        let y = y + i as u32;
        if y < draw_limit {
            for x in 0..10 {
                if ((1 << x) & *m) != 0 {
                    draw_pixel(frame, x + x_offset, y, color);
                }
            }
        }
    }
}