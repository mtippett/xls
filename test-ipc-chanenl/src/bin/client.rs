use ipc_channel::ipc::IpcSender;
use std::io;

fn main() {
    println!("Client Starting!");
    let stdin = io::stdin();
    let mut input = String::new();

    // Prompt and read for the pipe name
    println!("Enter pipe name:");
    stdin.read_line(&mut input).unwrap();

    // trim the newline
    input = input.trim().to_string();

    println!("Connecting to {:?}", input);
    let result = IpcSender::connect(input);

    // Report any errors
    match result {
        Ok(_) => println!("Connected"),
        Err(err) => {
            println!("error {}", err);
            return;
        }
    }

    println!("Connected!");
    let sender = result.unwrap();

    /*
    loop until we get a disconnect
    */
    loop {
        let mut message = String::new();
        stdin.read_line(&mut message).unwrap();
        message = message.trim().to_string();

        if message == "DISCONNECT" {
            println!("Got disconnect, exiting");
            break;
        }

        sender.send(message).unwrap();
    }
}
