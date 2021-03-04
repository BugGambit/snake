extern crate serialport;
use serialport::SerialPort;
use std::{io, string::FromUtf8Error, time::Duration};

use crate::{direction::Direction, point::Point};

pub fn setup_port(port_name: &str) -> Box<dyn SerialPort> {
    let mut port = serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    port.set_timeout(Duration::from_secs(10)).unwrap();
    port
}

pub fn read_joystick_as_direction(port: &mut Box<dyn SerialPort>) -> Option<Direction> {
    let point = match read_joystick_as_point(port) {
        Some(point) => point,
        None => return None,
    };
    let epsilon = 100;
    let max = 1023 - epsilon;
    let min = epsilon;
    let direction: Direction;
    if point.x > max {
        direction = Direction::SOUTH;
    } else if point.x < min {
        direction = Direction::NORTH;
    } else if point.y > max {
        direction = Direction::WEST;
    } else if point.y < min {
        direction = Direction::EAST;
    } else {
        return None;
    }
    Some(direction)
}

fn read_joystick_as_point(port: &mut Box<dyn SerialPort>) -> Option<Point> {
    let str = match read_line(port) {
        Ok(str) => str,
        Err(_) => return None,
    };
    if str.trim() == "" {
        return None;
    }
    parse_point(str)
}

fn parse_point(str: String) -> Option<Point> {
    let parts: Vec<&str> = str.split(",").collect();
    let x = match parts[0].parse::<i32>() {
        Ok(val) => val,
        Err(_) => return None,
    };
    let y = match parts[1].parse::<i32>() {
        Ok(val) => val,
        Err(_) => return None,
    };
    Some(Point { x, y })
}

fn read_line(port: &mut Box<dyn SerialPort>) -> Result<String, FromUtf8Error> {
    let mut str = String::new();
    loop {
        let mut serial_buf: Vec<u8> = vec![0; 1];
        port.read(serial_buf.as_mut_slice())
            .expect("Failed to read");
        if serial_buf[0] == b'\r' {
            // discard the next \n char
            port.read(serial_buf.as_mut_slice()).unwrap();
            break;
        }
        str.push_str(&String::from_utf8(serial_buf)?[..]);
    }
    Ok(str)
}

fn read_lines(port: &mut Box<dyn SerialPort>) -> io::Result<Vec<String>> {
    let mut str = String::from("");
    port.read_to_string(&mut str)?;
    let parts: Vec<String> = str.split("\r\n").map(|x| x.to_string()).collect();
    Ok(parts)
}
