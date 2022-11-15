
#[macro_use] mod common;
mod samples;

use rocket::{launch, get, routes, fs::NamedFile};
use std::{f32, string};
use std::path::PathBuf;
use rocket::FromForm;
use samples::Sample;



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
	NamedFile::open("prostate_cancer.html").await.ok()
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
	NamedFile::open(&format!("static/{}", file.display())).await.ok()
	//let path = path.into_inner();
	//if !is_path_safe(&path) { return not_found_404(); }
	//file_response(&format!("static/{}", &path), cache(7))
}




// #[get("/predict?<cfdna_yield>&<psa>&<ldh>&<alp>&<liver_mets>&<lung_mets>&<bone_mets>&<ecog>")]
// fn predict(cfdna_yield: Option<f32>, psa: Option<f32>,
// 	ldh: Option<f32>, alp: Option<f32>, liver_mets: Option<bool>,
// 	lung_mets: Option<bool>, bone_mets: Option<bool>, ecog: Option<&str>)
// 	-> Option<String> {
	
// 	let psa_weight: f32 = 0.0104819;
// 	let ldh_weight: f32 = 1.74115;
// 	let alp_weight: f32 = 0.366035;
// 	let cfdna_weight: f32 = 0.0419012;
// 	let ecog_weight: f32 = 1.24476;
// 	let lung_mets_weight: f32 = 2.61359;
// 	let liver_mets_weight: f32 = 2.85341;
// 	let bone_mets_weight: f32 = 1.89837;

// 	// Parse the query parameters, return 400 error if malformatted
// 	/*let cfdna_yield: Option<f32> = parse_query(&req, "cfdna_yield")?;
// 	let psa: Option<f32> = parse_query(&req, "psa")?;
// 	let ldh: Option<f32> = parse_query(&req, "ldh")?;
// 	let alp: Option<f32> = parse_query(&req, "alp")?;
// 	let liver_mets: Option<bool> = parse_query(&req, "liver_mets")?;
// 	let lung_mets: Option<bool> = parse_query(&req, "lung_mets")?;
// 	let bone_mets: Option<bool> = parse_query(&req, "bone_mets")?;*/

// 	// Parse the ECOG parameter. Valid values are: N/A, 0, 1, 2+
// 	let ecog: Option<u8> = match ecog {
// 		Some("0") => Some(0),
// 		Some("1") => Some(1),
// 		Some("2 ") => Some(2),  // "2+" gets converted to "2 " in decoding
// 		None => None,
// 		_ => {
// 			eprintln!("Invalid ECOG query: {}", ecog.unwrap());
// 			return None;
// 		}
// 	};

// 	// Find the nearest neighbors
// 	let distances: Vec<f32> = SAMPLES.iter().map(|s| {
// 		let mut dist: f32 = 0.0;
// 		dist += distance(cfdna_yield, s.cfdna_yield) * cfdna_weight;
// 		dist += distance(psa, s.psa) * psa_weight;
// 		dist += distance(ldh, s.ldh) * ldh_weight;
// 		dist += distance(alp, s.alp) * alp_weight;

// 		// For ECOG we need to do some extra checking
// 		if let Some(test) = ecog {
// 			if let Some(known) = s.ecog {
// 				if test < 2u8 || known < 2u8 {
// 					dist += ((test as f32) - (known as f32)).abs() * ecog_weight;
// 				} else {
// 					// Both ECOG are 2 or above => zero distance
// 				}
// 			} else {
// 				dist += f32::INFINITY;
// 			}
// 		}

// 		if let Some(liver_mets) = liver_mets {
// 			dist += match s.liver_mets {
// 				None => f32::INFINITY,
// 				Some(x) => if liver_mets == x { 0.0 } else { liver_mets_weight }
// 			};
// 		}

// 		if let Some(lung_mets) = lung_mets {
// 			dist += match s.lung_mets {
// 				None => f32::INFINITY,
// 				Some(x) => if lung_mets == x { 0.0 } else { lung_mets_weight }
// 			};
// 		}

// 		if let Some(bone_mets) = bone_mets {
// 			dist += match s.bone_mets {
// 				None => f32::INFINITY,
// 				Some(x) => if bone_mets == x { 0.0 } else { bone_mets_weight }
// 			};
// 		}

// 		dist
// 	}).collect();

// 	let mut nearest: Vec<usize> = (0..SAMPLES.len()).collect();
// 	nearest.sort_by(|&a, &b|
// 		distances[a].partial_cmp(&distances[b]).unwrap_or(Ordering::Equal));

// 	// Start with a 10 nearest neighbors model, but add more neighbors
// 	// if the next neighbors are equally close.
// 	let mut K = 10;
// 	let tied_distance = distances[nearest[K - 1]];
// 	while distances[nearest[K]] <= tied_distance { K += 1; }
// 	nearest.truncate(K);

// 	let mut nearest_ctdna: Vec<f32> = nearest.iter()
// 		.map(|&k| SAMPLES[k].ctdna_frac).collect();
// 	nearest_ctdna.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

// 	// Estimate ctDNA fraction as the median of the neighbors
// 	let ctdna_estimate = if K % 2 == 0 {
// 		(nearest_ctdna[K / 2 - 1] + nearest_ctdna[K / 2]) / 2.0
// 	} else {
// 		nearest_ctdna[K / 2]
// 	};

// 	// Send the ctDNA% point estimate and a table of nearest neighbors
// 	// to the client.
// 	let mut output = String::new();
// 	write!(output, "{:.1}\n", ctdna_estimate);
// 	for k in &nearest {
// 		let sample = &SAMPLES[*k];
// 		if let Some(x) = sample.cfdna_yield {
// 			write!(output, "{}\t", x); } else { write!(output, "\t"); }
// 		if let Some(x) = sample.psa {
// 			write!(output, "{}\t", x); } else { write!(output, "\t"); }
// 		if let Some(x) = sample.ldh {
// 			write!(output, "{:.2}\t", x); } else { write!(output, "\t"); }
// 		if let Some(x) = sample.alp {
// 			write!(output, "{:.2}\t", x); } else { write!(output, "\t"); }
// 		if let Some(x) = sample.ecog {
// 			write!(output, "{}\t", x); } else { write!(output, "\t"); }

// 		if let Some(x) = sample.bone_mets {
// 			write!(output, "{}\t", if x { "Yes" } else { "No" });
// 		} else {
// 			write!(output, "\t");
// 		}

// 		if let Some(x) = sample.lung_mets {
// 			write!(output, "{}\t", if x { "Yes" } else { "No" });
// 		} else {
// 			write!(output, "\t");
// 		}

// 		if let Some(x) = sample.liver_mets {
// 			write!(output, "{}\t", if x { "Yes" } else { "No" });
// 		} else {
// 			write!(output, "\t");
// 		}

// 		write!(output, "{:.1}\n", sample.ctdna_frac);
// 	}

// 	Some(output)
// }

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