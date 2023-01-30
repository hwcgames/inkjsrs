use inkjsrs::{Inklecate, Story};
use rand::seq::IteratorRandom;
#[test]
fn intercept() {
	let mut rng = rand::thread_rng();
	let mut inklecate = Inklecate::init();
	let compiled = inklecate.compile(&reqwest::blocking::get("https://raw.githubusercontent.com/inkle/the-intercept/master/Assets/Ink/TheIntercept.ink").unwrap().text().unwrap());
	let mut story = Story::new(compiled.clone());
	for line in story.continue_maximally() {
		println!("{}", line);
	}
	let saved = story.save();

	let mut story = Story::new(compiled);
	story.load(saved.clone());
	let saved_2 = story.save();

	assert_eq!(saved, saved_2);

	for _ in 0..1000 {
		'inner: loop {
			for line in story.continue_maximally() {
				println!("{}", line);
			}
			let choices = story.choices();
			for choice in &choices {
				println!("> {}", choice.text);
			}
			if let Some(choice) = (0..choices.len()).choose(&mut rng) {
				story.choose(choice);
			} else {
				println!("Finished!");
				break 'inner;
			}
		}
	}
}
