use web_sys::console;
use wasm_bindgen::prelude::*;
use std::convert::TryInto;

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



struct Random {
	a: u32, b: u32, c: u32, d: u32
}

impl Random {
	pub fn with_seed(seed: u32) -> Random {
		let mut r = Random { a: 0, b: 0, c: 0, d: 0 };
		r.init(seed);
		r
	}

	pub fn init(&mut self, seed: u32) {
		self.a = 0x6c078965 * (seed ^ (seed >> 30)) + 1;
		self.b = 0x6c078965 * (self.a ^ (self.a >> 30)) + 2;
		self.c = 0x6c078965 * (self.b ^ (self.b >> 30)) + 3;
		self.d = 0x6c078965 * (self.c ^ (self.c >> 30)) + 4;
	}

	pub fn roll(&mut self) -> u32 {
		let n = self.a ^ (self.a << 11);
		self.a = self.b;
		self.b = self.c;
		self.c = self.d;
		self.d = n ^ (n >> 8) ^ self.d ^ (self.d >> 19);
		self.d
	}

	pub fn roll_max(&mut self, limit: u32) -> u32 {
		let value = (self.roll() as u64) * (limit as u64);
		(value >> 32).try_into().unwrap()
	}
	pub fn roll_max8(&mut self, limit: u8) -> u8 {
		let value = (self.roll() as u64) * (limit as u64);
		(value >> 32).try_into().unwrap()
	}
}


fn from_linear_hour(linear_hour: u8) -> u8 {
	if linear_hour < 5 {
		19 + linear_hour
	} else {
		linear_hour - 5
	}
}
fn to_linear_hour(hour: u8) -> u8 {
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
/*
fn get_next_day(year: &mut u16, month: &mut u8, day: &mut u8) {
	let leap = (year & 3) == 0;
	day += 1;
	let month_len = if leap && month == 2 { 29 } else { MONTH_LENGTHS[(month - 1) as usize] };
	if (day - 1) == month_len {
		month += 1;
		day = 1;
		if month == 13 {
			month = 1;
			year += 1;
		}
	}
}
*/

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
	let mut rng = Random {
		a: (year as u32) << 8,
		b: (month as u32) << 8,
		c: (day as u32) << 8,
		d: seed | 0x80000000
	};
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
				let seed = seed + 0x80000000 + 0x1000 * ((year as u32) * 0x1000 + (month as u32) * 0x40 + (day as u32));
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

	let adjust_y = 0x2000000 * (year as u32);
	let adjust_m = 0x200000 * (month as u32);
	let adjust_d = 0x10000 * (day as u32);
	let mut rng = Random::with_seed(seed + 0x80000000 + adjust_y + adjust_m + adjust_d);
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
	let adjust_y = 0x2000000 * (year as u32);
	let adjust_m = 0x200000 * (month as u32);
	let adjust_d = 0x10000 * (day as u32);
	let adjust_h = hour as u32;
	let mut rng = Random::with_seed(seed + 0x80000000 + adjust_y + adjust_m + adjust_d + adjust_h);
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
	let mut rng = Random::with_seed(seed_base + (minute as u32) * 0x100);
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
	let adjust_y = 0x20000 * (year as u32);
	let adjust_m = 0x2000 * (month as u32);
	let adjust_d = 0x100 * (day as u32);
	let adjust_h = 0x10000 * (hour as u32);
	let seed = seed + 0x80000000 + adjust_y + adjust_m + adjust_d + adjust_h;
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

/*

bool southernFlag = false;

const uint64_t SECONDS_MASK = 0x0fffffffffffffff;
const uint64_t SECONDS_SHIFT = 60;


struct HourGuess {
	uint32_t hourSeedAdd;
	uint64_t trueMinuteMask;
	uint64_t falseMinuteMask;
	uint64_t secondMask[60];
};
struct DayGuess {
	uint32_t seedAdd;
	uint32_t rainbowSeedAdd;
	uint64_t patternMask;
	uint16_t dayIndex;
	uint8_t typeMask[24];
	uint8_t rainbowFlag;
	uint8_t specialDayFlag;

	uint32_t hourMask;
	HourGuess hours[9];
};

DayGuess dayGuesses[200];
int dayGuessCount = 0;

const uint64_t m1  = 0x5555555555555555;
const uint64_t m2  = 0x3333333333333333;
const uint64_t m4  = 0x0f0f0f0f0f0f0f0f;
const uint64_t h01 = 0x0101010101010101; 

inline int countOnBits(uint64_t x) {
    x -= (x >> 1) & m1;
    x = (x & m2) + ((x >> 2) & m2);
    x = (x + (x >> 4)) & m4;
    return (x * h01) >> 56;
}

EXPORT void guessClear() {
	dayGuessCount = 0;
}

int findDayGuess(int year, int month, int day) {
	uint32_t seedAdd = 0x80000000;
	seedAdd += 0x10000 * ((year * 0x200) + (month * 0x20) + day);
	for (int i = 0; i < dayGuessCount; i++) {
		if (dayGuesses[i].seedAdd == seedAdd)
			return i;
	}

	uint32_t rainbowSeedAdd = 0x80000000;
	rainbowSeedAdd += 0x1000 * (year * 0x1000 + month * 0x40 + day);

	dayGuesses[dayGuessCount].dayIndex = ((month - 1) * 31) + day - 1;
	dayGuesses[dayGuessCount].seedAdd = seedAdd;
	dayGuesses[dayGuessCount].rainbowSeedAdd = rainbowSeedAdd;
	dayGuesses[dayGuessCount].patternMask = 0;
	for (int i = 0; i < 24; i++)
		dayGuesses[dayGuessCount].typeMask[i] = 0;
	dayGuesses[dayGuessCount].rainbowFlag = 0;
	dayGuesses[dayGuessCount].specialDayFlag = isSpecialDay(southernFlag, year, month, day);
	dayGuesses[dayGuessCount].hourMask = 0;
	for (int linearHour = 0; linearHour < 9; linearHour++) {
		uint32_t hourSeedAdd = 0x80000000;
		hourSeedAdd += 0x10000 * fromLinearHour(linearHour);
		if (linearHour == 5)
			getNextDay(year, month, day);
		hourSeedAdd += 0x100 * ((year * 0x200) + (month * 0x20) + day);
		dayGuesses[dayGuessCount].hours[linearHour].hourSeedAdd = hourSeedAdd;
		dayGuesses[dayGuessCount].hours[linearHour].trueMinuteMask = 0;
		dayGuesses[dayGuessCount].hours[linearHour].falseMinuteMask = 0;
		for (int sec = 0; sec < 60; sec++) {
			dayGuesses[dayGuessCount].hours[linearHour].secondMask[sec] = 0;
		}
	}
	return dayGuessCount++;
}

EXPORT void guessAddType(int year, int month, int day, int hour, int type) {
	uint8_t mask = 1 << type;
	if (type == 98) {
		// no rain/snow
		mask = 1 << F;
		mask |= (1 << C);
		mask |= (1 << O);
		mask |= (1 << RC);
	} else if (type == 99) {
		mask = 1 << R;
		mask |= (1 << HR);
	}
	dayGuesses[findDayGuess(year, month, day)].typeMask[hour] |= mask;
}

EXPORT void guessAddPattern(int year, int month, int day, int pattern) {
	dayGuesses[findDayGuess(year, month, day)].patternMask |= (uint64_t(1) << pattern);
}

EXPORT bool guessAddMinute(int year, int month, int day, int hour, int minute, bool yes) {
	int linearHour = toLinearHour(hour);
	int g = findDayGuess(year, month, day);
	dayGuesses[g].hourMask |= (1 << linearHour);
	uint64_t mask = uint64_t(1) << minute;
	if (yes) {
		dayGuesses[g].hours[linearHour].trueMinuteMask |= mask;
		return ((dayGuesses[g].hours[linearHour].falseMinuteMask & mask) == 0);
	} else {
		dayGuesses[g].hours[linearHour].falseMinuteMask |= mask;
		return ((dayGuesses[g].hours[linearHour].trueMinuteMask & mask) == 0);
	}
}

EXPORT bool guessAddSecond(int year, int month, int day, int hour, int minute, int second) {
	int g = findDayGuess(year, month, day);
	if (guessAddMinute(year, month, day, hour, minute, true)) {
		int linearHour = toLinearHour(hour);
		uint64_t mask = uint64_t(1) << second;
		dayGuesses[g].hours[linearHour].secondMask[minute] |= mask;
		dayGuesses[g].hours[linearHour].secondMask[minute] &= SECONDS_MASK;
		uint64_t count = countOnBits(dayGuesses[g].hours[linearHour].secondMask[minute]);
		dayGuesses[g].hours[linearHour].secondMask[minute] |= (count << SECONDS_SHIFT);
		return true;
	}
	return false;
}

EXPORT void guessAddRainbowDouble(int year, int month, int day, bool isDouble) {
	dayGuesses[findDayGuess(year, month, day)].rainbowFlag = isDouble ? 2 : 1;
}




#define MAX_RESULTS 30
uint32_t results[MAX_RESULTS];
int resultCount = 0;
uint32_t searchPosition = 0;

EXPORT void searchInit(bool southern) {
	resultCount = 0;
	searchPosition = 0;
	southernFlag = southern;
}

EXPORT float searchGetPercentage() {
	return float(searchPosition) / float(0x7fffffff);
}
EXPORT bool searchCompleted() {
	return (searchPosition >= 0x7fffffff) || (resultCount >= MAX_RESULTS);
}
EXPORT bool searchFailed() {
	return (resultCount >= MAX_RESULTS) && (searchPosition < 0x7fffffff);
}

EXPORT int searchGetMaxResultCount() {
	return MAX_RESULTS;
}
EXPORT int searchGetResultCount() {
	return resultCount;
}
EXPORT uint32_t searchGetResult(int index) {
	return results[index];
}

bool checkGuess(uint32_t seed) {
	for (int g = 0; g < dayGuessCount; g++) {
		Random rng;
		uint8_t pattern;
		if (dayGuesses[g].specialDayFlag > 0) {
			pattern = EventDay00;
		} else {
			rng.init(seed + dayGuesses[g].seedAdd);
			rng.get();
			rng.get();
			uint16_t dayIndex = dayGuesses[g].dayIndex;
			uint8_t rateSet = southernFlag ? rateLookupS[dayIndex] : rateLookupN[dayIndex];
			pattern = rateMaps[rateSet][rng.get(100)];
		}

		// pattern check
		if (dayGuesses[g].patternMask != 0) {
			uint64_t check = uint64_t(1) << pattern;
			if ((dayGuesses[g].patternMask & check) == 0)
				return false;
		}

		// pattern contents check
		for (int i = 0; i < 24; i++) {
			uint8_t typeMask = dayGuesses[g].typeMask[i];
			uint8_t check = 1 << patterns[pattern][i];
			if (typeMask != 0 && (typeMask & check) == 0)
				return false;
		}

		// rainbow check
		if (dayGuesses[g].rainbowFlag != 0) {
			rng.init(seed + dayGuesses[g].rainbowSeedAdd);
			rng.get();
			rng.get();
			int rainbowCount = ((rng.get() & 1) == 0) ? 1 : 2;
			if (dayGuesses[g].rainbowFlag != rainbowCount)
				return false;
		}

		// minute check
		if (dayGuesses[g].hourMask != 0) {
			for (int linearHour = 0; linearHour < 9; linearHour++) {
				if ((dayGuesses[g].hourMask & (1 << linearHour)) == 0)
					continue;

				uint32_t hourSeed = seed + dayGuesses[g].hours[linearHour].hourSeedAdd;
				uint64_t trueMask = dayGuesses[g].hours[linearHour].trueMinuteMask;
				uint64_t falseMask = dayGuesses[g].hours[linearHour].falseMinuteMask;
				uint64_t mask = trueMask | falseMask;
				for (int minute = 0; minute < 60; minute++) {
					uint64_t check = uint64_t(1) << minute;
					if ((mask & check) != 0) {
						// do we expect to see meteor showers now?
						uint64_t expected = queryStarsInternal(pattern, hourSeed, linearHour, minute);
						if ((expected == 0) && ((trueMask & check) != 0))
							return false;
						if ((expected != 0) && ((falseMask & check) != 0))
							return false;

						uint64_t secondMask = dayGuesses[g].hours[linearHour].secondMask[minute];
						if (secondMask != 0) {
							/*int expectedCount = expected >> SECONDS_SHIFT;
							int guessesCount = secondMask >> SECONDS_SHIFT;
							if (expectedCount != guessesCount) return false;*/
							//int penalties = 0;
							for (int second = 0; second < 60; second++) {
								uint64_t check = uint64_t(1) << second;
								if ((secondMask & check) == 0) continue;

								if (second > 1) {
									check = uint64_t(1) << (second - 2);
									if ((expected & check) != 0) {
										expected ^= check;
										continue;
									}
								}
								if (second > 0) {
									check = uint64_t(1) << (second - 1);
									if ((expected & check) != 0) {
										expected ^= check;
										continue;
									}
								}
								check = uint64_t(1) << second;
								if ((expected & check) != 0) {
									expected ^= check;
								} else {
									return false;
								}

								/*check = uint64_t(1) << second;
								if ((secondMask & check) == 0) continue;
								if ((expected & check) == 0) {
									if (second > 0) check |= uint64_t(1) << (second - 1);
									if (second < 59) check |= uint64_t(1) << (second + 1);
									if ((expected & check) == 0) {
										if (second > 1) check |= uint64_t(1) << (second - 2);
										if (second < 58) check |= uint64_t(1) << (second + 2);
										if ((expected & check) == 0) {
											if (second > 2) check |= uint64_t(1) << (second - 3);
											if ((expected & check) == 0) {
												return false;
											} else {
												penalties += 3;
											}
										} else {
											penalties += 2;
										}
									} else {
										penalties += 1;
									}
								}
								if (penalties > 4) return false;*/
							}
						}
					}
				}
			}
		}
	}

	return true;
}

EXPORT void searchStep() {
	uint32_t stepEnd = searchPosition + 0x200000;
	if (stepEnd >= 0x7fffffff)
		stepEnd = 0x7fffffff;
	while (searchPosition < stepEnd && resultCount < MAX_RESULTS) {
		if (checkGuess(searchPosition)) {
			results[resultCount++] = searchPosition;
		}
		searchPosition++;
	}
}


*/
