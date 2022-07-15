use std::time::Instant;

#[derive(Clone, druid::Data)]
pub struct Stopwatch {
	#[data(ignore)]
	time: Instant,
	marker: String
}

impl Stopwatch {
	pub fn new() -> Stopwatch {
		Stopwatch {
			time: Instant::now(),
			marker: "undefined".to_string()
		}
	}
	pub fn start(&mut self, marker: &str) {
		self.marker = String::from(marker);
		self.time = Instant::now();
	}
	pub fn stop(&self) {
		let elapsed = self.time.elapsed().as_secs_f32();
		println!("[{}] in {} seconds", &self.marker, elapsed);
	}

	pub fn start_without_marker(&mut self) {
		self.marker = String::from("<unnamed stopwatch>");
		self.time = Instant::now();
	}
	pub fn stop_without_marker(&self) -> f32 {
		self.time.elapsed().as_secs_f32()
	}
}

