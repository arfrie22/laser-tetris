//! This example shows powerful PIO module in the RP2040 chip to communicate with WS2812 LED modules.
//! See (https://www.sparkfun.com/categories/tags/ws2812)

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::{clk_sys_freq, RoscRng};
use embassy_rp::gpio::Output;
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
        draw_mask(&mut data, 20, 7, p.y() + 2, p.mask(), c);

        let p = game.current_piece();
        draw_mask(&mut data, 20, 7, p.y() + 2, p.mask(), p.color());

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