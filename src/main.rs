use serialport::{
    open_with_settings, DataBits, FlowControl, Parity, SerialPortInfo, SerialPortSettings,
    SerialPortType, StopBits,
};
use std::env;

fn as_uart_label(portname: &str) -> String {
    match portname {
        "/dev/ttymxc0" => String::from("UART1"),
        "/dev/ttymxc1" => String::from("UART2"),
        "/dev/ttymxc2" => String::from("UART3"),
        "/dev/ttymxc3" => String::from("UART4"),
        _ => panic!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let loop_forever = args.iter().any(|arg| arg == "loop");
    let serial_to_use: Vec<String> = args
        .into_iter()
        .filter(|arg| arg.starts_with("/dev"))
        .collect();

    let settings = SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: std::time::Duration::from_millis(5000),
    };

    let serial_ports = if serial_to_use.is_empty() {
        serialport::available_ports().expect("No serial_ports found!")
    } else {
        serial_to_use
            .iter()
            .map(|p| SerialPortInfo {
                port_name: p.clone(),
                port_type: SerialPortType::Unknown,
            })
            .collect()
    };

    loop {
        for p in serial_ports.clone() {
            println!(
                "Trying in {} ({})",
                as_uart_label(&p.port_name),
                p.port_name
            );

            let mut serial = open_with_settings(&p.port_name, &settings).unwrap();

            serial.write(&[0xfe, 0x1, 0x41, 0, 0, 0x40]).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(500));

            let bytes_to_read = serial.bytes_to_read().unwrap();
            let mut buffer = vec![0x00; bytes_to_read as usize];

            serial.read_exact(&mut buffer).unwrap();
            println!("Bytes read {:?}\n", buffer);
        }
        if !loop_forever {
            break;
        }
    }
}
