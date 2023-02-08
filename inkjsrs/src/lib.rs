use js_sandbox::Script;
use serde::Deserialize;
use serde_json::Value;

const INK_SOURCE: &str = include_str!(env!("INKJS_PATH"));

const JS_INTERFACE: &str = include_str!("./interface.js");

/// The Ink compiler.
pub struct Inklecate {
	script: Script,
}

impl Inklecate {
	pub fn init() -> Inklecate {
		let script = Script::from_string(&(INK_SOURCE.to_owned() + JS_INTERFACE))
			.expect("Failed to initialize inkjs");

		Inklecate { script }
	}

	pub fn compile(&mut self, ink: &str) -> Value {
		self.script
			.call("compile", &ink)
			.expect("Compilation failed")
	}
}

pub struct Story {
	script: Script,
}

impl Story {
	pub fn new(ink: Value) -> Story {
		let mut script = Script::from_string(&(INK_SOURCE.to_owned() + JS_INTERFACE))
			.expect("Failed to initialize inkjs");
		let _: () = script.call("init_ink", &ink).expect("Failed to load story");

		Story { script }
	}

	pub fn can_continue(&mut self) -> bool {
		self.script.call("canContinue", &()).unwrap()
	}

	pub fn continue_once(&mut self) -> String {
		self.script.call("continueOnce", &()).unwrap()
	}

	pub fn continue_maximally(&mut self) -> Vec<String> {
		self.script.call("continueMaximally", &()).unwrap()
	}

	pub fn save(&mut self) -> String {
		self.script.call("save", &()).unwrap()
	}

	pub fn load(&mut self, saved: String) {
		self.script.call("load", &saved).unwrap()
	}

	pub fn choices(&mut self) -> Vec<Choice> {
		self.script.call("current_choices", &()).unwrap()
	}

	pub fn choose(&mut self, choice: usize) {
		self.script.call("choose", &choice).unwrap()
	}

	pub fn jump(&mut self, to: String) {
		self.script.call("jump", &to).unwrap()
	}

	pub fn set(&mut self, key: String, value: String) {
		self.script.call("set", &(key, value)).unwrap()
	}

	pub fn get(&mut self, key: String) -> String {
		self.script.call("set", &key).unwrap()
	}

	pub fn visited(&mut self, label: String) -> usize {
		self.script.call("visited", &label).unwrap()
	}
}

#[derive(Deserialize)]
pub struct Choice {
	pub text: String,
	pub index: u64,
	pub tags: Vec<String>,
}
