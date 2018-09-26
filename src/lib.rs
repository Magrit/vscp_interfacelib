
extern crate byteorder;

use std::io::Cursor;
use std::net::UdpSocket;
use std::string::String;
use std::io::ErrorKind;
use byteorder::{ReadBytesExt, LittleEndian};

#[derive (Copy)]
pub struct Packet {
// magic : u32 == 0xAABBCCDD
pub forward_backward : f32,
pub left_right :f32
}


impl Packet
{   
    pub fn deserialize(cursor : &mut Cursor<&mut[u8]>) -> Packet
    {
        let magic_u32 = cursor.read_u32::<LittleEndian>().unwrap();
        if magic_u32 == 0xAABBCCDD 
        {
            Packet
            {
                forward_backward:cursor.read_f32::<LittleEndian>().unwrap(),
                left_right:cursor.read_f32::<LittleEndian>().unwrap()
            }
        }else
        {
            println!("ERROR: INVALID MAGIC!");
            Packet
            {
                forward_backward: 0.0,
                left_right: 0.0
            }
        }
    }
}

pub struct Client
{
    sock : UdpSocket,
    last_packet : Packet
}

impl Client
{
    pub fn new(host : String) -> Client
    {
        let sock = UdpSocket::bind(host).expect("Could not bind hostname");
        sock.set_nonblocking(true)
        .expect("Failed to enter non-blocking mode");
        Client {
            sock: sock,
            last_packet: Packet{
                forward_backward: 0.0,
                left_right: 0.0}
        }
    }

    pub fn read_vscp(&mut self, buf : &mut [u8; 12]) -> Packet
    {
        let result = self.sock.recv(buf);
        match result {
            Ok(num_bytes) => {
                let mut cursor = Cursor::new(&mut buf[..num_bytes]);
                self.last_packet = Packet::deserialize(&mut cursor);
                self.last_packet
            },
            Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                println!("Something went wrong: {}", err);
                self.last_packet
            }, 
            _ => {
                self.last_packet
            }
        }
    }
}

// fn main() {

// let host = "10.0.0.45:50001";
// let client = Client::new(host.to_owned());
// let mut buf : [u8; 12] = [0; 12];
// loop
// {
//     let packet = client.read_vscp(&mut buf);
//     println!("Values sent are:  forward_backward:{:?}, left_right:{:?}", packet.forward_backward, packet.left_right);
// }

// }
