use std::net::{IpAddr, Ipv4Addr};

extern crate hap;
use hap::transport::Transport;
use hap::transport::ip::IpTransport;
use hap::accessory::{Information, outlet};
use hap::config::Config;

fn main() {
    let information = Information {
        name: "youcontrol Plug".into(),
        manufacturer: "youcontrol.io".into(),
        serial_number: "12345".into(),
        ..Default::default()
    };
    let outlet = outlet::new(information);

    let config = Config {
        name: "youcontrol_Plug".into(),
        ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 49)),
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new_device(config).unwrap();

    ip_transport.start().unwrap();
}
