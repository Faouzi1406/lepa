use "other.lp";

const var = 20;

fn wow() number {
	if  10 >= 10 {
		 if 2 > 10 {
			printf("2 is more then 10"); 
		 } else {
			printf("2 is not more then 10"); 
		 }
		 lol();
	}
	else {
		printf("this might seem like magic but 10 is more then 1 ... I know right???"); 
	}
	return 1;
}

fn wowie() {
	printf(" wowie."); 
	return;
}

fn main() {
	let num = 2;
	let other = 10;
	let some = other;
	{
		let wow = some;
		let huh = wow;
		{
			let kek = huh;
		}
	}
	wowie(num);
	return;
}
