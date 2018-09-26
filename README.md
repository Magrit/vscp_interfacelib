# vscp_interfacelib
Best vscpinterface ever

This is how to use it:
----
fn main() {

let host = "10.0.0.45:50001";
let mut client = Client::new(host.to_owned());
let mut buf : [u8; 12] = [0; 12];
loop
{
    let packet = client.read_vscp(&mut buf);
    println!("Values sent are:  forward_backward:{:?}, left_right:{:?}", packet.forward_backward, packet.left_right);
}

#Client

An UDP client that receives packets


# Packet

The packet struct is defined as:
forward_backward : f32
left_right : f32