// ! Hello World client
// #![crate_name = "helloworld_client"]
pub mod samples;
use protobuf::Message;
use samples::{Sample, Classification};
use std::any::type_name;
fn main() {
    // let file = File::open("/Users/wilsontu/Desktop/github/ctdna_predict/fl_clin_ctdna_data_Aug2022.tsv").expect("could not open file");


    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5556").is_ok());

    let mut msg = zmq::Message::new();
    // let mut out = Vec::new();
    // let mut writer = Writer::new(&mut out);
    for request_nbr in 0..10 {
        let mut sample = Sample::new();
        sample.cfDNA_ng_mL_plasma =  Some(request_nbr as f64);
        sample.Albumin =  Some(request_nbr as f64);
        sample.LDH =  Some(request_nbr as f64);
        sample.ALP =  Some(request_nbr as f64);
        sample.PSA =  Some(request_nbr as f64);
        sample.liver_met =  Some(request_nbr);
        sample.lung_met =  Some(request_nbr);
        sample.ecog =  Some(request_nbr);
        let mut class = Classification::new();
        let message = sample.write_to_bytes().unwrap();
        println!("Sending Hello {}...",  request_nbr);
        requester.send(message, 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        // println!("Received World {}\n", std::str::from_utf8(&mut msg).unwrap() );
        // Classification::parse_from_bytes(&mut msg).unwrap();
        let response = Classification::parse_from_bytes(&mut msg).unwrap();
        let is_bool  = response.label.unwrap();
        println!("Received World {}: {}", is_bool, request_nbr);
    }
}

// fn main (){
//     use std::env;
//     env::set_var("OUT_DIR", ".");
//     protobuf_codegen::Codegen::new()
//     // Use `protoc` parser, optional.
//     .protoc()
//     .includes(&["."])
//     // Use `protoc-bin-vendored` bundled protoc command, optional.
//     // .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
//     // Inputs must reside in some of include paths.
//     .input("./samples.proto")
//     // .input("src/protos/banana.proto")
//     // Specify output directory relative to Cargo output directory.
//     .cargo_out_dir("protos")
//     .run_from_script();
// }