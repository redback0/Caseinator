use std::env;
use rand::prelude::*;


fn main() {
	let args : Vec<String> = env::args().collect();
	assert_eq!(args.get(1).is_some(), true);
	println!("{}", args.get(1).unwrap_or(&String::from("LL")));
	let caseinated : String = caseinator(args.get(1).unwrap());

	println!("{}", caseinated);
}

/*
   fn caseinator(string : &String) -> String {
   let words = string.split_whitespace();

   let caseinated = caseinator_lower(words);

   return caseinated;
   }
 */

fn caseinator(string: &String) -> String {
	let mut rng = rand::rng();
	let caseinators = vec![
		lower_caseinator,
		upper_caseinator,
		camel_caseinator,
	];

	let rand_caseinator = caseinators.choose(&mut rng);
	return rand_caseinator.unwrap()(string);
	//return camel_caseinator(string);
}

fn lower_caseinator(string : &String) -> String {
	return string.to_lowercase();
}

fn upper_caseinator(string : &String) -> String {
	return string.to_uppercase();
}

fn camel_caseinator(string : &String) -> String {
	let mut words = string.split_whitespace();
	//let mut caseinated = String::new();
	let mut caseinated = String::from(words.nth(0).unwrap());

	for word in words {
		let mut temp = word.to_lowercase();
		temp.replace_range(
			temp
				.char_indices()
				.nth(0)
				.map(|(pos, ch)| pos..pos + ch.len_utf8())
				.unwrap(),
			&temp.chars().nth(0).unwrap().to_uppercase().to_string(),
		);
		caseinated.push_str(&temp);
	}
	return caseinated;
}
