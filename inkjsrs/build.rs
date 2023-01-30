use std::path::Path;

fn main() {
	println!("cargo:rerun-if-changed=inkjs.url");
	let result = reqwest::blocking::get(
		std::fs::read_to_string("./inkjs.url").expect("Couldn't read InkJS url"),
	)
	.expect("Failed to download InkJS")
	.text()
	.expect("Unpkg returned non-UTF8");

	let ink_path = Path::new(&std::env::var("OUT_DIR").expect("No out dir")).join("ink.js");

	std::fs::write(&ink_path, result).expect("Failed to write inkjs");

	println!("cargo:rustc-env=INKJS_PATH={}", ink_path.to_string_lossy());
}
