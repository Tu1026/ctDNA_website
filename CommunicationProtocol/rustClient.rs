
pub mod samples;
use protobuf::Message;
use samples::{Sample, Classification};
use std::any::type_name;

fn predict_ctDNA(sample: Sample) -> String {


    println!("Connecting to python ML host...\n");

    let context = zmq::Context::new();
    
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5556").is_ok(), "Failed to connect to server, python host might not be started");

    let mut msg = zmq::Message::new();

    // let mut sample = Sample::new();
    
    // sample.cfDNA_ng_mL_plasma =  Some(request_nbr as f64);
    // sample.Albumin =  Some(request_nbr as f64);
    // sample.LDH =  Some(request_nbr as f64);
    // sample.ALP =  Some(request_nbr as f64);
    // sample.PSA =  Some(request_nbr as f64);
    // sample.liver_met =  Some(request_nbr);
    // sample.lung_met =  Some(request_nbr);
    // sample.ecog =  Some(request_nbr);
    
    let mut class = Classification::new();
    let message = sample.write_to_bytes().unwrap();

    requester.send(message, 0).unwrap();

    requester.recv(&mut msg, 0).unwrap();
    // println!("Received World {}\n", std::str::from_utf8(&mut msg).unwrap() );
    // Classification::parse_from_bytes(&mut msg).unwrap();
    let response = Classification::parse_from_bytes(&mut msg).unwrap();
    let is_bool  = response.label.unwrap();
    println!("Receive {}", is_bool);

}