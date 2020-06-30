extern crate meteonook;
use meteonook::*;

fn main() {
	use Pattern::*;

	let mut gd = GuessData::new(Hemisphere::Northern);

	gd.add_pattern(2020, 4, 4, Fine00);
	gd.add_pattern(2020, 4, 4, Fine02);
	gd.add_pattern(2020, 4, 4, Fine04);
	gd.add_pattern(2020, 4, 4, Fine06);

	gd.add_pattern(2020, 4, 16, Fine00);
	gd.add_pattern(2020, 4, 16, Fine02);
	gd.add_pattern(2020, 4, 16, Fine04);
	gd.add_pattern(2020, 4, 16, Fine06);

	gd.add_pattern(2020, 4, 19, CloudFine00);
	gd.add_rainbow(2020, 4, 19, false);

	gd.add_pattern(2020, 6, 7, Fine00);

	gd.add_pattern(2020, 6, 16, Fine00);
	gd.add_pattern(2020, 6, 16, Fine02);
	gd.add_pattern(2020, 6, 16, Fine04);
	gd.add_pattern(2020, 6, 16, Fine06);
	gd.add_pattern(2020, 6, 16, Cloud00);
	gd.add_pattern(2020, 6, 16, Cloud01);
	gd.add_pattern(2020, 6, 16, Cloud02);
	gd.add_pattern(2020, 6, 16, Rain00);
	gd.add_pattern(2020, 6, 16, FineCloud00);
	gd.add_pattern(2020, 6, 16, FineCloud01);
	gd.add_pattern(2020, 6, 16, CloudFine00);
	gd.add_pattern(2020, 6, 16, CloudFine01);
	gd.add_pattern(2020, 6, 16, CloudFine02);
	gd.add_pattern(2020, 6, 16, RainCloud00);
	gd.add_pattern(2020, 6, 16, RainCloud01);
	gd.add_pattern(2020, 6, 16, RainCloud02);

	gd.add_pattern(2020, 6, 17, Rain01);
	gd.add_pattern(2020, 6, 17, CloudRain01);

	gd.add_pattern(2020, 6, 18, Rain02);
	gd.add_pattern(2020, 6, 18, CloudFine00);

	gd.add_pattern(2020, 6, 19, Fine02);
	gd.add_pattern(2020, 6, 19, Fine04);
	gd.add_pattern(2020, 6, 19, Fine06);

	gd.add_pattern(2020, 6, 20, Cloud02);
	gd.add_pattern(2020, 6, 20, RainCloud02);

	gd.add_pattern(2020, 6, 29, Fine00);

	let mut guesser = Guesser::new(0, 0x7fffffff);
	loop {
		match guesser.work(&gd, 0x200001) {
			GuesserResult::Complete => break,
			GuesserResult::Incomplete => (),
			GuesserResult::Failed => break
		};
		let percentage = guesser.get_percentage();
		println!("{}... results={}", percentage, guesser.get_result_count());
	}
	let res = guesser.work(&gd, 0x7fffffff);
	println!("result: {:?}", res);

	for i in 0..guesser.get_result_count() {
		let seed = guesser.get_result(i);
		println!("seed {}: {}", i, seed);
	}
}
