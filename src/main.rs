extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusClient, ModbusRTU};

use std::{thread, time};

const SERVER_ID: u8 = 1;

fn main()
{
    println!("Modbus RTU Example");

    let mut modbus: Modbus;
    modbus = Modbus::new_rtu("/dev/ttyUSB1", 115200, 'N', 8, 1).unwrap();
    modbus.set_slave(SERVER_ID).unwrap();

    modbus.set_debug(true).unwrap();

    match modbus.connect()
    {
        Err(err) => panic!("Connection failed: {}", err),
        Ok(_) => {}
    }

    let response_timeout = modbus.get_response_timeout().expect("could not get response timeout");
    println!("response_timeout : {:?}\n", response_timeout);

    let mut tab_rp_registers = vec![0u16; 1024 as usize];

    // Single register

    let time_ms = time::Duration::from_millis(500);

    let mut value = 0;
    loop
    {
        value = value + 1;
        let _rc = modbus.write_register(0, value);
        thread::sleep(time_ms);

        let _rc = modbus.read_registers(0, 1, &mut tab_rp_registers).unwrap();
        thread::sleep(time_ms);
    }
}
