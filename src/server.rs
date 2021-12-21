use crate::world_structs;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
use std::str::from_utf8;
use serde_json;
fn handle(mut stream: TcpStream, world: Arc<Mutex<world_structs::World>>) {

    let mut entities: Vec<world_structs::Entity> = Vec::new();
    let mut compare_time = SystemTime::now();
    let mut data = [0 as u8; 256];
    let mut world_clone = world.lock().unwrap();
    while match stream.read(&mut data) {
        Ok(_size) => {
            // network stuff
            let res = match from_utf8(&data) {
                Ok(_v) => _v,
                Err(e) => panic!("Invalid sequence: {}", e),
            }.replace("\0", "").replace("\n", "").to_string();
            let res_obj: world_structs::WorldRequest = match serde_json::from_str(&res) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            };
            if res_obj.req_type == world_structs::RequestType::CHUNK {
                let response = world_structs::WorldResponse {
                    chunk: world_clone.chunks[res_obj.x as usize][res_obj.y as usize].clone(),
                    entities: world_clone.get_entities_for_chunk(world_clone.chunks[res_obj.x as usize][res_obj.y as usize].clone())
                };
                let msg = serde_json::to_string(&response).unwrap();

                stream.write(msg.as_bytes()).unwrap();
            }

            else if res_obj.req_type == world_structs::RequestType::DATA{
                let msg = serde_json::to_string(&world_clone.world_data).unwrap();
                stream.write(msg.as_bytes()).unwrap();
            }
            // game tick
            
            let delta = SystemTime::now().duration_since(compare_time).unwrap();
            let _delta_as_millis = delta.as_millis()/10;
                if delta.as_millis()/10 != 0 {
                 //   println!("FPS: {}", 100 / (delta.as_millis()/10));
                }

                        
            // edit entities
                world_clone.update_entities("move".to_string());
            // end stuff
            compare_time = SystemTime::now();

            data = [0 as u8; 256];
            true
        },
        Err(_) => {
            println!("Error occurred, closing connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
/*fn move(e: &mut world_structs::Entity) {
    e.x += 0.1;
}*/
pub fn serve(world: world_structs::World, _port: i32) {
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
        println!("Server listening on port 5000");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {

                    let world_clone = world.clone();
                    println!("New connection with: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle(stream, Arc::new(Mutex::new(world_clone)));
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        drop(listener); 

    });
}
