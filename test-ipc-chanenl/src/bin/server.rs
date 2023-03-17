use ipc_channel::ipc;


fn main() {
    println!("Server Starting!");
    let (server, server_path) = ipc::IpcOneShotServer::<String>::new().unwrap();

    println!("Server Accepting Connections on {}",server_path);
    let (receiver,_) = server.accept().unwrap();
    println!("Server Connection");

    loop{
        match receiver.recv() {
            Ok(message) => println!("Received message {}",message),
            Err(error) => {
                println!("Error occured: {:?}", error);
                break;
            }
        }        
    }

    println!("Server done");
}
