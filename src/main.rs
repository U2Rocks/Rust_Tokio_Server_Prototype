// ***RUST CHAT SERVER***
// Connect with client(WORKING) -> send message to stream(WIP) -> end server

// NEXT GOAL: make server not end automatically

// import statements
use std::fs;
use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

// struct that holds messages and their owner
struct MessageList {
    Messages: Vec<(String, String)>,
    CreationDate: f32,
    LastUpdate: f32,
}

// struct that holds ip address and stream for manipulation
struct AddressHolder {
    stream: Option<TcpStream>,
    socket_ip: Option<SocketAddr>,
}

// function that modifies sockAddr vector to add new ip
fn add_to_ip_list(holder: &AddressHolder, list: &mut Vec<SocketAddr>) {
    let new_ip = holder.socket_ip;
    list.push(new_ip.unwrap());
}

// function that handles streams/connections
async fn handle_connection(input_stream: TcpStream) -> Option<tokio::net::TcpStream> {
    // get html file data
    let contents = fs::read_to_string("../home.html").unwrap();
    // create response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    let conv_response = response.as_bytes();
    // write to stream
    let _waiter = input_stream.writable().await; // this pauses code until stream can be written too
    match input_stream.try_write(conv_response) {
        Ok(_n) => println!("Successful write to client"),
        Err(_e) => println!("Failed write to client"),
    }

    // return TcpStream to original caller
    return Some(input_stream); // must return option to please struct
}

// use async main instead of normal main function for Rust***

#[tokio::main]
async fn main() -> io::Result<()> {
    // create ip list
    let mut ip_list: Vec<SocketAddr> = vec![];
    // create message list
    let mut record_list = MessageList {
        Messages: vec![(String::from("Chatter1"), String::from("Hello Guys"))],
        CreationDate: 1001600.2023,
        LastUpdate: 1001600.2023,
    };
    // create tcp listener/ip_list/AddressHolder and panic if error occurs(?)
    let listener = TcpListener::bind("127.0.0.1:2347").await?;
    let mut con1 = AddressHolder {
        stream: None,
        socket_ip: None,
    };

    // match addressholder return and send error message if not working
    match listener.accept().await {
        Ok((socket, addr)) => {
            con1 = AddressHolder {
                stream: Some(socket),
                socket_ip: Some(addr),
            }
        }
        Err(e) => println!("couldn't get client: {:?}", e),
    };

    // write to tcp stream using handle_connection function
    con1.stream = handle_connection(con1.stream.unwrap()).await; // await is necessary here or only future is returned

    // add ip to list
    add_to_ip_list(&con1, &mut ip_list);

    // print length of list
    println!("Length of ip_list is {}", ip_list.len());

    // print out message before ending server
    println!(
        "BEFORE SHUTDOWN: The address [{:?}] was connected to this server. extra: [{:?}]",
        con1.socket_ip.unwrap(),
        con1.stream.unwrap()
    );

    // print ip list
    println!("BEFORE SHUTDOWN: ip list: {:?}", ip_list);

    Ok(())
}
