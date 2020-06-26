use web_sys::console;
use wasm_bindgen::prelude::*;
use std::convert::TryInto;
use std::num::Wrapping;
use std::cmp;

macro_rules! log {
	( $( $t:tt )* ) => {
		web_sys::console::log_1(&format!( $( $t )* ).into());
	}
}

mod data;
pub use data::*;

#[wasm_bindgen(js_name = isSpecialDay)]
pub fn is_special_day(hemi: Hemisphere, y: u16, m: u8, d: u8) -> SpecialDay {
	use SpecialDay::*;
	let y: usize = y.into();

	if y >= 2000 && y <= 2060 {
		if y == 2020 && m == EASTER_MONTHS[y - 2000] && d == EASTER_DAYS[y - 2000] {
			return Easter;
		}
		if m == 1 && d == FISH_CON_JAN[y - 2000] { return FishCon; }
		if m == 4 && d == FISH_CON_APR[y - 2000] { return FishCon; }
		if m == 7 && d == FISH_CON_JUL[y - 2000] { return FishCon; }
		if m == 10 && d == FISH_CON_OCT[y - 2000] { return FishCon; }
		match hemi {
			Hemisphere::Northern => {
				if m == 6 && d == INSECT_CON_JUN_N[y - 2000] { return InsectCon; }
				if m == 7 && d == INSECT_CON_JUL_N[y - 2000] { return InsectCon; }
				if m == 8 && d == INSECT_CON_AUG_N[y - 2000] { return InsectCon; }
				if m == 9 && d == INSECT_CON_SEP_N[y - 2000] { return InsectCon; }
			}
			Hemisphere::Southern => {
				if m == 1 && d == INSECT_CON_JAN_S[y - 2000] { return InsectCon; }
				if m == 2 && d == INSECT_CON_FEB_S[y - 2000] { return InsectCon; }
				if m == 3 && d == INSECT_CON_MAR_S[y - 2000] { return InsectCon; }
				if m == 12 && d == INSECT_CON_DEC_S[y - 2000] { return InsectCon; }
			}
		}
	}
	if m == 12 && d == 31 { return Countdown; }
	return None;
}



fn compute_seed_ymd(base: u32, year_mult: u32, month_mult: u32, day_mult: u32, year: u16, month: u8, day: u8) -> u32 {
	let y = year_mult.wrapping_mul(year as u32);
	let m = month_mult.wrapping_mul(month as u32);
	let d = day_mult.wrapping_mul(day as u32);
	(base | 0x80000000).wrapping_add(y).wrapping_add(m).wrapping_add(d)
}
fn compute_seed_ymdh(base: u32, year_mult: u32, month_mult: u32, day_mult: u32, hour_mult: u32, year: u16, month: u8, day: u8, hour: u8) -> u32 {
	let seed = compute_seed_ymd(base, year_mult, month_mult, day_mult, year, month, day);
	let h = hour_mult.wrapping_mul(hour as u32);
	seed.wrapping_add(h)
}

#[wasm_bindgen]
pub struct Random {
	a: Wrapping<u32>, b: Wrapping<u32>, c: Wrapping<u32>, d: Wrapping<u32>
}

#[wasm_bindgen]
impl Random {
	#[wasm_bindgen(js_name = withState)]
	pub fn with_state(a: u32, b: u32, c: u32, d: u32) -> Random {
		Random { a: Wrapping(a), b: Wrapping(b), c: Wrapping(c), d: Wrapping(d) }
	}

	#[wasm_bindgen(js_name = withSeed)]
	pub fn with_seed(seed: u32) -> Random {
		let mut r = Random { a: Wrapping(0), b: Wrapping(0), c: Wrapping(0), d: Wrapping(0) };
		r.init(seed);
		r
	}

	pub fn init(&mut self, seed: u32) {
		let seed = Wrapping(seed);
		let mult = Wrapping(0x6c078965u32);
		self.a = (seed ^ (seed >> 30)) * mult + Wrapping(1);
		self.b = (self.a ^ (self.a >> 30)) * mult + Wrapping(2);
		self.c = (self.b ^ (self.b >> 30)) * mult + Wrapping(3);
		self.d = (self.c ^ (self.c >> 30)) * mult + Wrapping(4);
	}

	pub fn roll(&mut self) -> u32 {
		let n = self.a ^ (self.a << 11);
		self.a = self.b;
		self.b = self.c;
		self.c = self.d;
		self.d = n ^ (n >> 8) ^ self.d ^ (self.d >> 19);
		self.d.0
	}

	#[wasm_bindgen(js_name = rollMax)]
	pub fn roll_max(&mut self, limit: u32) -> u32 {
		let value = (self.roll() as u64) * (limit as u64);
		(value >> 32).try_into().unwrap()
	}
	pub fn roll_max8(&mut self, limit: u8) -> u8 {
		let value = (self.roll() as u64) * (limit as u64);
		(value >> 32).try_into().unwrap()
	}
}


#[wasm_bindgen(js_name = fromLinearHour)]
pub fn from_linear_hour(linear_hour: u8) -> u8 {
	if linear_hour < 5 {
		19 + linear_hour
	} else {
		linear_hour - 5
	}
}
#[wasm_bindgen(js_name = toLinearHour)]
pub fn to_linear_hour(hour: u8) -> u8 {
	if hour >= 19 {
		hour - 19
	} else {
		hour + 5
	}
}

static MONTH_LENGTHS: [u8;12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[wasm_bindgen(js_name = getMonthLength)]
pub fn get_month_length(year: u16, month: u8) -> u8 {
	let leap = (year & 3) == 0;
	if leap && month == 2 {
		29
	} else {
		MONTH_LENGTHS[(month - 1) as usize]
	}
}


fn get_next_day(year: u16, month: u8, day: u8) -> (u16, u8, u8) {
	let mut year = year;
	let mut month = month;
	let mut day = day;
	day += 1;
	if day > get_month_length(year, month) {
		month += 1;
		day = 1;
		if month > 12 {
			month = 1;
			year += 1;
		}
	}
	(year, month, day)
}

fn normalise_late_ymd(year: u16, month: u8, day: u8, hour: u8) -> (u16, u8, u8) {
	if hour < 5 {
		get_next_day(year, month, day)
	} else {
		(year, month, day)
	}
}

#[wasm_bindgen(js_name = getSnowLevel)]
pub fn get_snow_level(hemi: Hemisphere, month: u8, day: u8) -> SnowLevel {
	use Hemisphere::*;
	use SnowLevel::*;
	match (hemi, month, day) {
		(Northern, 11, 26..=30) => Low,
		(Northern, 12,  1..=10) => Low,
		(Northern, 12, 11..=31) => Full,
		(Northern,  1,       _) => Full,
		(Northern,  2,  1..=24) => Full,

		(Southern,  5, 26..=31) => Low,
		(Southern,  6,  1..=10) => Low,
		(Southern,  6, 11..=30) => Full,
		(Southern,  7,       _) => Full,
		(Southern,  8,  1..=24) => Full,

		_ => None
	}
}

#[wasm_bindgen(js_name = getCloudLevel)]
pub fn get_cloud_level(hemi: Hemisphere, month: u8, day: u8) -> CloudLevel {
	use Hemisphere::*;
	use CloudLevel::*;
	match (hemi, month, day) {
		(Northern,  7, 21..=31) => Cumulonimbus,
		(Northern,  8,       _) => Cumulonimbus,
		(Northern,  9,  1..=15) => Cumulonimbus,
		(Northern,  9, 16..=30) => Cirrus,
		(Northern, 10,       _) => Cirrus,
		(Northern, 11,       _) => Cirrus,
		(Northern, 12,       _) => Billow,
		(Northern,  1,       _) => Billow,
		(Northern,  2,       _) => Billow,
		(Northern,  3,       _) => Thin,
		(Northern,  4,       _) => Thin,
		(Northern,  5,       _) => Thin,

		(Southern,  1, 21..=31) => Cumulonimbus,
		(Southern,  2,       _) => Cumulonimbus,
		(Southern,  3,  1..=15) => Cumulonimbus,
		(Southern,  3, 16..=31) => Cirrus,
		(Southern,  4,       _) => Cirrus,
		(Southern,  5,       _) => Cirrus,
		(Southern,  6,       _) => Billow,
		(Southern,  7,       _) => Billow,
		(Southern,  8,       _) => Billow,
		(Southern,  9,       _) => Thin,
		(Southern, 10,       _) => Thin,
		(Southern, 11,       _) => Thin,

		_ => None
	}
}

#[wasm_bindgen(js_name = getSpWeatherLevel)]
pub fn get_sp_weather_level(hemi: Hemisphere, month: u8, day: u8) -> SpWeatherLevel {
	use Hemisphere::*;
	use SpWeatherLevel::*;
	match (hemi, month, day) {
		(Northern,      12, 11..=31) => Aurora,
		(Northern,       1,       _) => Aurora,
		(Northern,       2,  1..=24) => Aurora,
		(Northern,       2, 25..=29) => Rainbow,
		(Northern,  3..=10,       _) => Rainbow,
		(Northern,      11,  1..=25) => Rainbow,

		(Southern,       6, 11..=30) => Aurora,
		(Southern,       7,       _) => Aurora,
		(Southern,       8,  1..=24) => Aurora,
		(Southern,       8, 25..=31) => Rainbow,
		(Southern,  9..=12,       _) => Rainbow,
		(Southern,  1..= 4,       _) => Rainbow,
		(Southern,       5,  1..=25) => Rainbow,

		_ => None
	}
}

#[wasm_bindgen(js_name = getFogLevel)]
pub fn get_fog_level(hemi: Hemisphere, month: u8, day: u8) -> FogLevel {
	// 1N: 21/9 to 24/2 - heavy fog, river and sea fog
	// 2N: 25/2 to 31/3 - river and sea fog
	// 1S: 21/3 to 24/8
	// 2S: 25/8 to 30/9
	use Hemisphere::*;
	use FogLevel::*;
	match (hemi, month, day) {
		(Northern,       9, 21..=30) => HeavyAndWater,
		(Northern, 10..=12,       _) => HeavyAndWater,
		(Northern,       1,       _) => HeavyAndWater,
		(Northern,       2,  1..=24) => HeavyAndWater,
		(Northern,       2, 25..=29) => WaterOnly,
		(Northern,       3,       _) => WaterOnly,

		(Southern,       3, 21..=31) => HeavyAndWater,
		(Southern,  4..= 7,       _) => HeavyAndWater,
		(Southern,       8,  1..=24) => HeavyAndWater,
		(Southern,       8, 25..=31) => WaterOnly,
		(Southern,       9,       _) => WaterOnly,

		_ => None
	}
}

#[wasm_bindgen(js_name = checkWaterFog)]
pub fn check_water_fog(seed: u32, year: u16, month: u8, day: u8) -> bool {
	let mut rng = Random::with_state(
		(year as u32) << 8,
		(month as u32) << 8,
		(day as u32) << 8,
		seed | 0x80000000u32
	);
	rng.roll();
	rng.roll();
	(rng.roll() & 1) == 1
}


#[wasm_bindgen]
pub struct RainbowInfo { pub count: u8, pub hour: u8 }

#[wasm_bindgen(js_name = getRainbowInfo)]
pub fn get_rainbow_info(hemi: Hemisphere, seed: u32, year: u16, month: u8, day: u8, pattern: Pattern) -> RainbowInfo {
	if get_sp_weather_level(hemi, month, day) == SpWeatherLevel::Rainbow {
		match pattern.kind() {
			PatternKind::CloudFine | PatternKind::FineRain => {
				let seed = compute_seed_ymd(seed, 0x1000000, 0x40000, 0x1000, year, month, day);
				let mut rng = Random::with_seed(seed);
				rng.roll();
				rng.roll();
				let rainbow_count = match rng.roll() & 1 {
					0 => 1,
					1 => 2,
					_ => 0 // should never happen
				};
				for h in 7..=17 {
					use Weather::*;
					let a = PATTERNS[pattern as usize][h];
					let b = PATTERNS[pattern as usize][h + 1];
					if (a == Rain || a == HeavyRain) && (b == Clear || b == Sunny) {
						return RainbowInfo { count: rainbow_count, hour: (h + 1) as u8 }
					}
				}
			}
			_ => ()
		}
	}

	RainbowInfo { count: 0, hour: 0 }
}

#[wasm_bindgen(js_name = isAuroraPattern)]
pub fn is_aurora_pattern(hemi: Hemisphere, month: u8, day: u8, pattern: Pattern) -> bool {
	use Pattern::*;
	match get_sp_weather_level(hemi, month, day) {
		SpWeatherLevel::Aurora => match pattern {
			Fine01 | Fine03 | Fine05 => true,
			_ => false
		}
		_ => false
	}
}

#[wasm_bindgen(js_name = getPattern)]
pub fn get_pattern(hemi: Hemisphere, seed: u32, year: u16, month: u8, day: u8) -> Pattern {
	if is_special_day(hemi, year, month, day) != SpecialDay::None {
		return Pattern::EventDay00;
	}

	let seed = compute_seed_ymd(seed, 0x2000000, 0x200000, 0x10000, year, month, day);
	let mut rng = Random::with_seed(seed);
	rng.roll();
	rng.roll();
	let rate_set = match hemi {
		Hemisphere::Northern => RATE_LOOKUP_N[(month - 1) as usize][(day - 1) as usize],
		Hemisphere::Southern => RATE_LOOKUP_S[(month - 1) as usize][(day - 1) as usize]
	};
	Pattern::from_u8(RATE_MAPS[rate_set as usize][rng.roll_max(100) as usize])
}

#[wasm_bindgen(js_name = isPatternPossibleAtDate)]
pub fn is_pattern_possible_at_date(hemi: Hemisphere, month: u8, day: u8, pattern: Pattern) -> bool {
	let rate_set = match hemi {
		Hemisphere::Northern => RATE_LOOKUP_N[(month - 1) as usize][(day - 1) as usize],
		Hemisphere::Southern => RATE_LOOKUP_S[(month - 1) as usize][(day - 1) as usize]
	};
	for p in RATE_MAPS[rate_set as usize].iter() {
		if *p == (pattern as u8) {
			return true
		}
	}
	false
}


#[wasm_bindgen(js_name = getWeather)]
pub fn get_weather(hour: u8, pattern: Pattern) -> Weather {
	PATTERNS[pattern as usize][hour as usize]
}

#[wasm_bindgen(js_name = isHeavyShowerPattern)]
pub fn is_heavy_shower_pattern(pattern: Pattern) -> bool {
	pattern == Pattern::Fine00
}
#[wasm_bindgen(js_name = isLightShowerPattern)]
pub fn is_light_shower_pattern(pattern: Pattern) -> bool {
	use Pattern::*;
	match pattern {
		Fine02 | Fine04 | Fine06 => true,
		_ => false
	}
}


#[wasm_bindgen(js_name = getWindPower)]
pub fn get_wind_power(seed: u32, year: u16, month: u8, day: u8, hour: u8, pattern: Pattern) -> u8 {
	use WindType::*;
	let (year, month, day) = normalise_late_ymd(year, month, day, hour);
	let seed = compute_seed_ymdh(seed, 0x2000000, 0x200000, 0x10000, 1, year, month, day, hour);
	let mut rng = Random::with_seed(seed);
	rng.roll();
	rng.roll();
	match WINDS[pattern as usize][hour as usize] {
		Calm => 0,
		Land0 | Sea0 => rng.roll_max8(3),
		Land1 | Sea1 => rng.roll_max8(4) + 1,
		Land2 | Sea2 => rng.roll_max8(3) + 3
	}
}


#[wasm_bindgen(js_name = getWindPowerMin)]
pub fn get_wind_power_min(hour: u8, pattern: Pattern) -> u8 {
	use WindType::*;
	match WINDS[pattern as usize][hour as usize] {
		Calm => 0,
		Land0 | Sea0 => 0,
		Land1 | Sea1 => 1,
		Land2 | Sea2 => 3
	}
}
#[wasm_bindgen(js_name = getWindPowerMax)]
pub fn get_wind_power_max(hour: u8, pattern: Pattern) -> u8 {
	use WindType::*;
	match WINDS[pattern as usize][hour as usize] {
		Calm => 0,
		Land0 | Sea0 => 3,
		Land1 | Sea1 => 4,
		Land2 | Sea2 => 5
	}
}

#[wasm_bindgen(js_name = canHaveShootingStars)]
pub fn can_have_shooting_stars(hour: u8, pattern: Pattern) -> bool {
	if hour >= 19 || hour < 4 {
		use Pattern::*;
		match pattern {
			Fine00 => true,
			Fine02 | Fine04 | Fine06 => get_weather(hour, pattern) == Weather::Clear,
			_ => false
		}
	} else {
		false
	}
}


fn query_stars_internal(seed_base: u32, minute: u8, pattern: Pattern) -> Option<(u8, u64)> {
	let mut rng = Random::with_seed(seed_base.wrapping_add((minute as u32) * 0x100));
	let star_count = match pattern {
		Pattern::Fine00 => {
			// heavy shower
			let ok = rng.roll_max(100) < 50;
			if !ok { return None }
			if rng.roll_max(100) < 50 {
				8
			} else {
				5
			}
		}
		Pattern::Fine02 | Pattern::Fine04 | Pattern::Fine06 => {
			// light shower
			let chance = if (minute & 1) == 0 { 2 } else { 4 };
			let ok = rng.roll_max(60) < chance;
			if !ok { return None }
			5
		}
		_ => return None
	};

	let mut star_field = 0u64;
	let mut remaining = star_count;
	while remaining > 0 {
		let bit = rng.roll_max(60);
		let mask = 1u64 << bit;
		if (star_field & mask) == 0 {
			star_field |= mask;
			remaining -= 1;
		}
	}
	Some((star_count, star_field))
}



static mut LAST_STAR_SECONDS: [u8;8] = [0;8];

#[wasm_bindgen(js_name = queryStars)]
pub fn query_stars(seed: u32, year: u16, month: u8, day: u8, hour: u8, minute: u8, pattern: Pattern) -> u8 {
	let (year, month, day) = normalise_late_ymd(year, month, day, hour);
	let seed = compute_seed_ymdh(seed, 0x20000, 0x2000, 0x100, 0x10000, year, month, day, hour);
	match query_stars_internal(seed, minute, pattern) {
		None => 0,
		Some((star_count, star_field)) => {
			let mut index = 0;
			for second in 0..60 {
				let mask = 1u64 << second;
				if (star_field & mask) != 0 {
					unsafe { LAST_STAR_SECONDS[index] = second; }
					index += 1;
				}
			}
			star_count
		}
	}
}
#[wasm_bindgen(js_name = getStarSecond)]
pub fn get_star_second(index: usize) -> u8 {
	unsafe { LAST_STAR_SECONDS[index] }
}




#[derive(Clone, Copy)]
struct HourGuess {
	hour_seed_add: u32,
	true_minute_mask: u64,
	false_minute_mask: u64,
	second_mask: [u64;60]
}
impl Default for HourGuess {
	fn default() -> Self {
		HourGuess {
			hour_seed_add: 0,
			true_minute_mask: 0,
			false_minute_mask: 0,
			second_mask: [0u64;60]
		}
	}
}

#[derive(Default, Clone, Copy)]
struct DayGuess {
	seed_add: u32,
	rainbow_seed_add: u32,
	pattern_mask: u64,
	month: u8,
	day: u8,
	rainbow_count: u8,
	special_day_flag: bool,
	hour_mask: u16,
	hours: [HourGuess;9]
}

impl DayGuess {
	pub fn check(&self, seed: u32, pattern: Pattern) -> bool {
		// pattern check
		let pattern_bit = 1u64 << (pattern as u8);
		if (self.pattern_mask & pattern_bit) == 0 {
			if seed == 1856402561 { log!("seedfail pattern"); }
			return false;
		}

		// rainbow check
		if self.rainbow_count > 0 {
			let mut rng = Random::with_seed(seed.wrapping_add(self.rainbow_seed_add));
			rng.roll();
			rng.roll();
			let rainbow_count = match rng.roll() & 1 {
				0 => 1,
				1 => 2,
				_ => 0 // should never happen
			};
			if self.rainbow_count != rainbow_count {
				if seed == 1856402561 { log!("seedfail rainbow"); }
				return false;
			}
		}

		// meteor shower check
		if self.hour_mask != 0 {
			// for the sake of rollover, we need to process hours with data
			// and also the hour right before
			let hour_filter = self.hour_mask | (self.hour_mask >> 1);
			let mut previous_second_mask = 0u64;

			for linear_hour in 0..9 {
				let hour_bit = 1u16 << linear_hour;
				if (hour_filter & hour_bit) == 0 {
					// no data for this hour
					continue;
				}

				let hg = &self.hours[linear_hour as usize];
				let hour_seed = seed.wrapping_add(hg.hour_seed_add);
				let true_mask = hg.true_minute_mask;
				let false_mask = hg.false_minute_mask;

				// save time by only calculating specific minutes:
				//   minutes with data
				//   minutes before minutes with data (for roll-over)
				//   last minute of the hour (also for roll-over)
				// technically, the last won't always be needed
				// but it doesn't hurt and makes things a little simpler for now
				let minute_filter = (true_mask | false_mask) | ((true_mask | false_mask) >> 1) | (1u64 << 59);

				for minute in 0..60 {
					let minute_bit = 1u64 << minute;
					if (minute_filter & minute_bit) == 0 {
						// no data for this minute
						continue
					}

					match query_stars_internal(hour_seed, minute, pattern) {
						None => {
							// no stars
							if (true_mask & minute_bit) != 0 && previous_second_mask == 0 {
								if seed == 1856402561 { log!("seedfail stars false positive at {}:{}", from_linear_hour(linear_hour), minute); }
								return false
							}
							previous_second_mask = 0;
						}
						Some((_star_count, star_field)) => {
							// some stars
							if (false_mask & minute_bit) != 0 {
								if seed == 1856402561 { log!("seedfail stars false negative"); }
								return false
							}

							// do per-second checks
							if hg.second_mask[minute as usize] != 0 {
								// create u64 bitfield containing last 4 seconds of previous minute
								// (at the lowest position) followed by 60 seconds of current minute
								let mut actual_mask = (star_field << 4) | previous_second_mask;

								// these are the seconds that the user claimed they saw a star in
								let input_mask = hg.second_mask[minute as usize];

								let mut second_bit = 1u64;
								for _second in 0..60 {
									if (input_mask & second_bit) != 0 {
										// check 5 seconds worth for a match
										let mut check_bit = second_bit;
										let mut found = false;
										for _i in 0..5 {
											if (actual_mask & check_bit) != 0 {
												// found a match, remove it from consideration
												actual_mask ^= check_bit;
												found = true;
												break;
											}
											check_bit <<= 1;
										}
										if !found {
											// no match within 5 seconds distance, can't be true
											return false;
										}
									}
									second_bit <<= 1;
								}
							}

							// rollover the last 4 seconds
							previous_second_mask = star_field >> 56;
						}
					}
				}
			}
		}

		true
	}
}

#[wasm_bindgen]
pub struct GuessData {
	hemisphere: Hemisphere,
	days: [DayGuess;200],
	count: usize
}

#[wasm_bindgen]
impl GuessData {
	pub fn new(hemisphere: Hemisphere) -> GuessData {
		GuessData {
			hemisphere,
			days: [Default::default();200],
			count: 0
		}
	}

	fn find_day(&mut self, year: u16, month: u8, day: u8) -> Option<&mut DayGuess> {
		let seed_add = compute_seed_ymd(0, 0x2000000, 0x200000, 0x10000, year, month, day);
		for i in 0..self.count {
			if self.days[i].seed_add == seed_add {
				return Some(&mut self.days[i]);
			}
		}
		if self.count >= 200 {
			return None;
		}

		let dg = &mut self.days[self.count];
		dg.month = month;
		dg.day = day;
		dg.seed_add = seed_add;
		dg.rainbow_seed_add = compute_seed_ymd(0, 0x1000000, 0x40000, 0x1000, year, month, day);
		dg.special_day_flag = is_special_day(self.hemisphere, year, month, day) != SpecialDay::None;
		for linear_hour in 0..9 {
			let hour = from_linear_hour(linear_hour);
			let (n_year, n_month, n_day) = normalise_late_ymd(year, month, day, hour);
			dg.hours[linear_hour as usize].hour_seed_add = compute_seed_ymdh(0, 0x20000, 0x2000, 0x100, 0x10000, n_year, n_month, n_day, hour);
		}
		self.count += 1;
		Some(dg)
	}

	#[wasm_bindgen(js_name = addPattern)]
	pub fn add_pattern(&mut self, year: u16, month: u8, day: u8, pat: Pattern) -> bool {
		match self.find_day(year, month, day) {
			None => false,
			Some(dg) => {
				//log!("+pat {}-{}-{} = {:?}", year, month, day, pat);
				dg.pattern_mask |= 1u64 << (pat as u32);
				true
			}
		}
	}

	#[wasm_bindgen(js_name = addMinute)]
	pub fn add_minute(&mut self, year: u16, month: u8, day: u8, hour: u8, minute: u8, yes: bool) -> bool {
		match self.find_day(year, month, day) {
			None => false,
			Some(dg) => {
				let linear_hour = to_linear_hour(hour);
				let hour_idx = linear_hour as usize;
				dg.hour_mask |= 1u16 << linear_hour;
				let bit = 1u64 << minute;
				// log!("+min {}-{}-{} {}:{} {}", year, month, day, hour, minute, yes);
				if yes {
					dg.hours[hour_idx].true_minute_mask |= bit;
					(dg.hours[hour_idx].false_minute_mask & bit) == 0
				} else {
					dg.hours[hour_idx].false_minute_mask |= bit;
					(dg.hours[hour_idx].true_minute_mask & bit) == 0
				}
			}
		}
	}

	#[wasm_bindgen(js_name = addSecond)]
	pub fn add_second(&mut self, year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> bool {
		if !self.add_minute(year, month, day, hour, minute, true) {
			return false
		}

		match self.find_day(year, month, day) {
			None => false,
			Some(dg) => {
				// log!("+sec {}-{}-{} {}:{}:{}", year, month, day, hour, minute, second);
				let linear_hour = to_linear_hour(hour);
				let hour_idx = linear_hour as usize;
				let minute_idx = minute as usize;
				let bit = 1u64 << second;
				dg.hours[hour_idx].second_mask[minute_idx] |= bit;
				true
			}
		}
	}

	#[wasm_bindgen(js_name = addRainbow)]
	pub fn add_rainbow(&mut self, year: u16, month: u8, day: u8, is_double: bool) -> bool {
		match self.find_day(year, month, day) {
			None => false,
			Some(dg) => {
				dg.rainbow_count = if is_double { 2 } else { 1 };
				true
			}
		}
	}

	pub fn check(&self, seed: u32) -> bool {
		for i in 0..self.count {
			let dg = &self.days[i];
			let pattern = if dg.special_day_flag {
				Pattern::EventDay00
			} else {
				let mut rng = Random::with_seed(seed.wrapping_add(dg.seed_add));
				rng.roll();
				rng.roll();
				let rate_set = match self.hemisphere {
					Hemisphere::Northern => RATE_LOOKUP_N[(dg.month - 1) as usize][(dg.day - 1) as usize],
					Hemisphere::Southern => RATE_LOOKUP_S[(dg.month - 1) as usize][(dg.day - 1) as usize]
				};
				Pattern::from_u8(RATE_MAPS[rate_set as usize][rng.roll_max(100) as usize])
			};

			if !dg.check(seed, pattern) {
				return false
			}
		}

		// if none of the guesses failed, then this seed's OK
		true
	}
}



#[wasm_bindgen]
pub struct Guesser {
	minimum: u32,
	maximum: u32,
	step: u32,
	results: [u32;30],
	result_count: usize
}

#[wasm_bindgen]
impl Guesser {
	pub fn new(minimum: u32, maximum: u32) -> Guesser {
		Guesser {
			minimum, maximum,
			step: minimum,
			results: [0u32;30],
			result_count: 0
		}
	}

	pub fn work(&mut self, data: &GuessData, step_size: u32) -> bool {
		let step_start = self.step;
		let step_end = cmp::min(self.maximum, self.step.saturating_add(step_size - 1));
		self.step = step_end + 1;

		for seed in step_start..=step_end {
			if data.check(seed) {
				if self.result_count < 30 {
					self.results[self.result_count] = seed;
					self.result_count += 1;
				} else {
					return false
				}
			}
		}

		true
	}

	#[wasm_bindgen(js_name = getPercentage)]
	pub fn get_percentage(&self) -> f32 {
		let range = (self.maximum - self.minimum) as f32;
		let step = (self.step - self.minimum) as f32;
		(step / range) * 100.
	}

	#[wasm_bindgen(js_name = getResultCount)]
	pub fn get_result_count(&self) -> usize {
		return self.result_count;
	}
	#[wasm_bindgen(js_name = getResult)]
	pub fn get_result(&self, index: usize) -> u32 {
		return self.results[index];
	}
}
