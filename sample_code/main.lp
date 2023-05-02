use "other.lp";

let var = 20;

fn wow() {
	printf("Isn't this so strange...! number: %d", var); 
	lol();
	return;
}

fn main() {
	let other = 2;
        wow();
	return;
}
