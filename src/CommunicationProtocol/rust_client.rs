
use protobuf::Message;
use crate::CommunicationProtocol::samples::{Sample, Classification};

pub fn predict_ctDNA(sample: Sample) -> String {


    println!("Connecting to python ML host...\n");

    let context = zmq::Context::new();
    
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5556").is_ok(), "Failed to connect to server, python host might not be started");

    let mut msg = zmq::Message::new();


    let mut class = Classification::new();
    let message = sample.write_to_bytes().unwrap();

    requester.send(message, 0).unwrap();

    requester.recv(&mut msg, 0).unwrap();
    // println!("Received World {}\n", std::str::from_utf8(&mut msg).unwrap() );
    // Classification::parse_from_bytes(&mut msg).unwrap();
    let response = Classification::parse_from_bytes(&mut msg).unwrap();
    let is_positive  = response.label.unwrap();
    println!("Receive {:}", response);

    if is_positive {
        format!(">2\n{}", response.positive_proba.unwrap().to_string())
    } else {
        format!("<=2\n{}", response.negative_proba.unwrap().to_string())
    }

}