
extern crate byteorder;

pub mod vscp{

use std::io::Cursor;
use std::net::UdpSocket;
use std::string::String;
use byteorder::{ReadBytesExt, LittleEndian};

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
        sock : UdpSocket
    }

    impl Client
    {
        pub fn new(host : String) -> Client
        {
            Client {
                sock: UdpSocket::bind(host).expect("Could not bind hostname: {:?}")
            }
        }

        pub fn read_vscp(&self, buf : &mut [u8; 12]) -> Packet
        {
            let number_of_bytes = self.sock.recv(buf)
            .expect("Did not receive data!");
            let mut cursor = Cursor::new(&mut buf[..number_of_bytes]);
            Packet::deserialize(&mut cursor)
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
