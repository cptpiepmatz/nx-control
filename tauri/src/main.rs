#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod protocol;

use std::error::Error;
use std::io::Write;
use std::net::{TcpStream, UdpSocket};
use std::{process, thread};
use std::time::Duration;
use gilrs::{Axis, Button, Event, EventType, Gilrs};
use crate::protocol::{Action, DirectInput, StickDirectInput};

fn main() -> Result<(), Box<dyn Error>> {
    //tauri::Builder::default()
    //  .run(tauri::generate_context!())
    //  .expect("error while running tauri application");

    let socket = UdpSocket::bind("0.0.0.0:5656").expect("couldn't bind to address");
    socket.connect("192.168.178.103:5656").expect("connect failed");

    let mut gilrs = Gilrs::new().unwrap();

    let mut input: DirectInput = Default::default();
    loop {
        while let Some(Event {id, event, time}) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                EventType::ButtonPressed(button, code) => {
                    println!("{button:?}: {code}");
                    match button {
                        Button::North => input.x = true,
                        Button::West => input.y = true,
                        Button::East => input.a = true,
                        Button::South => input.b = true,

                        Button::DPadLeft => input.dpad_left = true,
                        Button::DPadRight => input.dpad_right = true,
                        Button::DPadUp => input.dpad_up = true,
                        Button::DPadDown => input.dpad_down = true,

                        Button::LeftThumb => input.l_stick.pressed = true,
                        Button::RightThumb => input.r_stick.pressed = true,

                        Button::LeftTrigger => input.l = true,
                        Button::LeftTrigger2 => input.zl = true,
                        Button::RightTrigger => input.r = true,
                        Button::RightTrigger2 => input.zr = true,

                        Button::Select => input.minus = true,
                        Button::Start => input.plus = true,
                        Button::Mode => input.home = true,

                        _ => ()
                    };
                },
                EventType::ButtonReleased(button, code) => {
                    println!("{button:?}: {code}");
                    match button {
                        Button::North => input.x = false,
                        Button::West => input.y = false,
                        Button::East => input.a = false,
                        Button::South => input.b = false,

                        Button::DPadLeft => input.dpad_left = false,
                        Button::DPadRight => input.dpad_right = false,
                        Button::DPadUp => input.dpad_up = false,
                        Button::DPadDown => input.dpad_down = false,

                        Button::LeftThumb => input.l_stick.pressed = false,
                        Button::RightThumb => input.r_stick.pressed = false,

                        Button::LeftTrigger => input.l = false,
                        Button::LeftTrigger2 => input.zl = false,
                        Button::RightTrigger => input.r = false,
                        Button::RightTrigger2 => input.zr = false,

                        Button::Select => input.minus = false,
                        Button::Start => input.plus = false,
                        Button::Mode => input.home = false,
                        _ => ()
                    };
                },
                EventType::AxisChanged(axis, val, _) => {
                    match axis {
                        Axis::LeftStickX => input.l_stick.x = val,
                        Axis::LeftStickY => input.l_stick.y = val,
                        Axis::RightStickX => input.r_stick.x = val,
                        Axis::RightStickY => input.r_stick.y = val,
                        _ => ()
                    }
                },
                _ => ()
            };
        }
        socket.send(&Action::DirectInput(&input).to_bytes())?;
        thread::sleep(Duration::from_millis(7));
    }

    Ok(())
}
