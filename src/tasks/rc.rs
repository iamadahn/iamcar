use defmt::*;
use embassy_stm32::gpio::Output;
use embassy_stm32::spi::Spi;
use embassy_stm32::mode::Blocking;
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Sender;
use embedded_nrf24l01::*;
use crate::data_types::InputData;

const PAYLOAD_LENGTH: usize = 10;

#[embassy_executor::task]
pub async fn rc_controller_task(
    spi: Spi<'static, Blocking>,
    ce: Output<'static>,
    cns: Output<'static>,
    input_pub: Sender<'static, NoopRawMutex, InputData, 1>) {
    info!("Starting remote conroller task");
    let mut nrf = NRF24L01::new(ce, cns, spi).unwrap();
    nrf.set_frequency(8).unwrap();
    nrf.set_auto_retransmit(15, 15).unwrap();
    nrf.set_rf(&DataRate::R2Mbps, 0).unwrap();
    nrf.set_pipes_rx_enable(&[true, false, false, false, false, false]).unwrap();
    nrf.set_auto_ack(&[true, false, false, false, false, false]).unwrap();
    nrf.set_pipes_rx_lengths(&[None; 6]).unwrap();
    nrf.set_crc(CrcMode::TwoBytes).unwrap();
    nrf.set_rx_addr(0, &b"fnord"[..]).unwrap();
    nrf.set_tx_addr(&b"fnord"[..]).unwrap();
    nrf.flush_rx().unwrap();
    nrf.flush_tx().unwrap();

    let mut nrf = nrf.rx().unwrap();

    loop {
        if nrf.can_read().unwrap().is_some() {
            let payload = nrf.read().unwrap();
            if payload.len() != PAYLOAD_LENGTH {
                continue;
            }
            if !checksum_valid(payload.as_ref())
            {
                info!("Invalid checksum.");
                continue;
            }
            info!("Rc controller: bytes {}", payload.as_ref());
            if let Some(input) = payload_get_input(payload.as_ref()) {
                input_pub.send(input).await;
            }
        }
        Timer::after_millis(100).await;
    }
}

fn checksum_valid(payload: &[u8]) -> bool {
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;
    for byte in 0..payload.len() - 2 {
        ck_a = ck_a.wrapping_add(payload[byte]);
        ck_b = ck_b.wrapping_add(ck_a);
    }

    if (payload[PAYLOAD_LENGTH - 2], payload[PAYLOAD_LENGTH - 1]) != (ck_a, ck_b) {
        return false;
    }

    true
}

fn payload_get_input(payload: &[u8]) -> Option<InputData> {
    if (payload[0], payload[1], payload[2], payload[3]) != (0xDE, 0xAD, 0xBA, 0xBE) {
        return None;
    }

    let input = InputData {
        x1: payload[4],
        y1: payload[5],
        x2: payload[6],
        y2: payload[7],
    };

    Some(input)
}

