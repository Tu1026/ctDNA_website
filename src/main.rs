
#[macro_use] mod common;
pub mod CommunicationProtocol;
use rocket::{launch, get, routes, fs::NamedFile};
use std::{f32, string, env};
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
	println!("albumin: {}", sample.albumin.unwrap().to_string());
	return sample.albumin.unwrap().to_string();
	// sample.unwrap().ldh.unwrap().to_string().map_err(Debug)
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
	let test = format!("static/{}", file.display());
	let base = env!("ROCKET_FRONT_END_FILES"); 
	let fina =  base.to_owned() + &test;
	println!("file: {}", fina);
	NamedFile::open(env!("ROCKET_FRONT_END_FILES").to_owned()+ &format!("static/{}", file.display())).await.ok()
	// NamedFile::open(&format!("front_end/static/{}", file.display())).await.ok()

	//let path = path.into_inner();
	//if !is_path_safe(&path) { return not_found_404(); }
	//file_response(&format!("static/{}", &path), cache(7))
}


fn parse_bool(val: &str) -> Option<bool> {
	match val {
		"Yes" | "yes" | "true" => Some(true),
		"No" | "no" | "false" => Some(false),
		_ => None
	}
}





#[launch]
fn rocket() -> _ {
	// eprintln!("Read {} samples from database.", SAMPLES.len());
    rocket::build().mount("/", routes![
    	index, static_file, predict
    ])
}

/*
fn main() {
	HttpServer::new(|| App::new()
		.wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
		.service(index)
		.service(static_file)
		.route(predict_ctdna_frac)
	).bind("127.0.0.1:8368")
	.unwrap_or_else(|_| error!("Cannot bind port 8368.")).run();
}*/