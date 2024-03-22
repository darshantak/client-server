use std::io::prelude::*;
use std::net::TcpStream;

fn main(){
    #![allow(unused)]
    const TCP_SERVER_ADDRESS : &str = "localhost:1234";
    if let Ok(mut stream) = TcpStream::connect(TCP_SERVER_ADDRESS){
        //Connecting to the server
        println!("{} {}",stream.local_addr().unwrap().ip(),stream.local_addr().unwrap().port());
        let mut request : &str = "Hello hopeless";
        

        //Writing to the server
        stream.write(request.as_bytes());
        stream.flush();
        println!("Sent message is: {}",request);
        //Reading from the server
        let mut response_bytes = [0;1024];
        let resp = stream.read(&mut response_bytes);
        let message = String::from_utf8_lossy(&mut response_bytes);
        println!("Received Message is : {}",message);
    }
    else {
        println!("Error connecting the server.");
    }
}