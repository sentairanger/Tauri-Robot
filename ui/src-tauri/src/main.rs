// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rppal::gpio::Gpio;
use std::error::Error;
use std::time::Duration;
use std::thread;

const MOTOR_PIN_1: u8 = 13;
const MOTOR_PIN_2: u8 = 21;
const MOTOR_PIN_3: u8 = 17;
const MOTOR_PIN_4: u8 = 27;

#[tauri::command]
fn forward() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_high();
    pin2.set_low();
    pin3.set_high();
    pin4.set_low();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
}

#[tauri::command]
fn backward() -> Result<(), Box<dyn Error>> {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_low();
    pin2.set_high();
    pin3.set_low();
    pin4.set_high();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
  
}

#[tauri::command]
fn left() {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_low();
    pin2.set_high();
    pin3.set_high();
    pin4.set_low();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())  
}

#[tauri::command]
fn right() {
    let mut pin1 = Gpio::new()?.get(MOTOR_PIN_1)?.into_output();
    let mut pin2 = Gpio::new()?.get(MOTOR_PIN_2)?.into_output();
    let mut pin3 = Gpio::new()?.get(MOTOR_PIN_3)?.into_output();
    let mut pin4 = Gpio::new()?.get(MOTOR_PIN_4)?.into_output();
    

    pin1.set_high();
    pin2.set_low();
    pin3.set_low();
    pin4.set_high();
    thread::sleep(Duration::from_millis(1000));
    pin1.set_low();
    pin2.set_low();
    pin3.set_low();
    pin4.set_low();
    Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![forward, backward, left, right])
    .run(tauri::generate_context!("../../ui/src-tauri/tauri.conf.json"))
    .expect("error while running tauri application");
}
