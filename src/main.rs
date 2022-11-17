
#[macro_use] mod common;
pub mod CommunicationProtocol;
use rocket::{launch, get, routes, fs::NamedFile};
use std::{f32,  env};
use std::path::PathBuf;
use rocket::FromForm;
use CommunicationProtocol::samples::Sample;
use CommunicationProtocol::rust_client::predict_ctDNA;

#[derive(FromForm, Debug)]
struct Sample_web {
	// #[field(default = -1.0)]
	cfdna_yield: Option<f64>,
	// #[field(default = -1.0)]
	psa: Option<f64>,
	// #[field(default = -1.0)]
	ldh: Option<f64>,
	// #[field(default = -1.0)]
	alp: Option<f64>,
	// #[field(default = -1.0)]
	albumin: Option<f64>,
	// #[field(default = -1)]
	ecog: Option<i32>,
	// #[field(default = -1)]
	liver_mets: Option<i32>,
	// #[field(default = -1)]
	lung_mets: Option<i32>,
}

impl Sample_web {
	fn to_proto_sample(&self) -> Sample {
		let mut sample = Sample::new();
		sample.cfdna_yield = self.cfdna_yield;
		sample.psa = self.psa;
		sample.ldh = self.ldh;
		sample.alp = self.alp;
		sample.albumin = self.albumin;
		sample.ecog = self.ecog;
		sample.liver_mets = self.liver_mets;
		sample.lung_mets = self.lung_mets;
		sample
	}
}
	


#[get("/")]
async fn index() -> Option<NamedFile> {
	// NamedFile::open("front_end/prostate_cancer.html").await.ok()
	NamedFile::open(env!("ROCKET_FRONT_END_FILES").to_owned() + "prostate_cancer.html").await.ok()

}


#[get("/predict?<sample..>")]
fn predict(sample: Option<Sample_web>) -> String{
	let result = sample.unwrap();
	let sample= result.to_proto_sample();
	println!("{:?}", sample);
	let prediction_string = predict_ctDNA(sample);
	println!{"prediction_string: {}", prediction_string};
	prediction_string

}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(env!("ROCKET_FRONT_END_FILES").to_owned()+ &format!("static/{}", file.display())).await.ok()

}





#[launch]
fn rocket() -> _ {
	// eprintln!("Read {} samples from database.", SAMPLES.len());
    rocket::build().mount("/", routes![
    	index, static_file, predict
    ])
}

