use gdnative::prelude::*;
use inkjsrs::Choice;
use serde_json::Value;

#[derive(NativeClass)]
#[inherit(Object)]
pub struct Story {
	pub story: Option<inkjsrs::Story>,
}

#[methods]
impl Story {
	fn new(_owner: &Object) -> Self {
		Story { story: None }
	}

	pub fn insert_script(&mut self, src: String) {
		let story: Value = serde_json::from_str(&src).expect("Bad JSON");
		self.story = Some(inkjsrs::Story::new(story));
	}

	pub fn can_continue(&mut self) -> bool {
		self.story.as_mut().unwrap().can_continue()
	}

	pub fn continue_once(&mut self) -> String {
		self.story.as_mut().unwrap().continue_once()
	}

	pub fn continue_maximally(&mut self) -> Vec<String> {
		self.story.as_mut().unwrap().continue_maximally()
	}

	pub fn save(&mut self) -> String {
		self.story.as_mut().unwrap().save()
	}

	pub fn load(&mut self, saved: String) {
		self.story.as_mut().unwrap().load(saved)
	}

	pub fn choices(&mut self) -> Vec<Choice> {
		self.story.as_mut().unwrap().choices()
	}

	pub fn choose(&mut self, choice: usize) {
		self.story.as_mut().unwrap().choose(choice)
	}

	pub fn jump(&mut self, to: String) {
		self.story.as_mut().unwrap().jump(to)
	}

	pub fn set(&mut self, key: String, value: String) {
		self.story.as_mut().unwrap().set(key, value)
	}

	pub fn get(&mut self, key: String) -> String {
		self.story.as_mut().unwrap().get(key)
	}

	pub fn visited(&mut self, label: String) -> usize {
		self.story.as_mut().unwrap().visited(label)
	}
}

fn init(handle: InitHandle) {
	handle.add_class::<Story>();
}

godot_init!(init);
