import { Forecast, Hemisphere, MonthForecast, DayForecast, DayInfo, createDayInfo, populateGuessData, DayType, ShowerType, AmbiguousWeather, StarInfo } from './model'
import { Guesser, GuessData, GuesserResult, Random, SpecialDay, Weather, Pattern } from '../pkg'
import translations from './translations'

const ambiguousWeatherTypes = {
	[Weather.Clear]: [AmbiguousWeather.NoRain, AmbiguousWeather.ClearOrSunny],
	[Weather.Sunny]: [AmbiguousWeather.NoRain, AmbiguousWeather.ClearOrSunny, AmbiguousWeather.SunnyOrCloudy],
	[Weather.Cloudy]: [AmbiguousWeather.NoRain, AmbiguousWeather.SunnyOrCloudy, AmbiguousWeather.CloudyOrRainClouds],
	[Weather.RainClouds]: [AmbiguousWeather.NoRain, AmbiguousWeather.CloudyOrRainClouds],
	[Weather.Rain]: [AmbiguousWeather.RainOrHeavyRain],
	[Weather.HeavyRain]: [AmbiguousWeather.RainOrHeavyRain]
}

function makeStarInfo(hour: number, minute: number): StarInfo {
	if (minute >= 60) {
		hour += 1
		minute -= 60
	}
	if (hour >= 24) {
		hour -= 24
	}
	return {hour, minute, seconds: []}
}

function makeTestDay(rng: Random, forecast: DayForecast): DayInfo {
	const info = createDayInfo(forecast.date)

	if (forecast.rainbowCount > 0 && rng.rollMax(100) < 20) {
		// throw in a rainbow
		info.dayType = DayType.Rainbow
		info.rainbowDouble = (forecast.rainbowCount == 2)
		info.rainbowTime = forecast.rainbowHour
		return info
	}
	if (forecast.aurora && rng.rollMax(100) < 30) {
		// throw in aurora
		info.dayType = DayType.Aurora
		info.auroraFine01 = (rng.rollMax(100) < 40) ? true : (forecast.pattern == Pattern.Fine01)
		info.auroraFine03 = (rng.rollMax(100) < 40) ? true : (forecast.pattern == Pattern.Fine03)
		info.auroraFine05 = (rng.rollMax(100) < 40) ? true : (forecast.pattern == Pattern.Fine05)
		return info
	}

	if (forecast.heavyShower) {
		info.dayType = DayType.Shower
		info.showerType = ShowerType.Heavy
	} else if (forecast.lightShower) {
		info.dayType = DayType.Shower
	} else if (rng.rollMax(100) < 20) {
		info.dayType = DayType.None
	}

	for (const star of forecast.shootingStars) {
		if (rng.rollMax(100) < 25) {
			if (star.seconds.find(value => value > 57) !== undefined) {
				if (rng.rollMax(100) < 30 && (star.hour != 3 || star.minute != 59)) {
					// record a star in the wrong minute
					info.stars.push(makeStarInfo(star.hour, star.minute + 1))
					continue
				}
			}
			const starInfo = makeStarInfo(star.hour, star.minute)
			if (rng.rollMax(100) < 20) {
				// put in the seconds
				for (const second of star.seconds) {
					if (second == 99)
						continue
					const blockFuzzing = (second > 56 && star.hour == 3 && star.minute == 59)
					const fuzzAmount = (!blockFuzzing && (rng.rollMax(100) < 70)) ? rng.rollMax(4) : 0
					const fuzzSecond = second + fuzzAmount
					if (fuzzSecond >= 60) {
						const badStarInfo = makeStarInfo(star.hour, star.minute + 1)
						badStarInfo.seconds.push(fuzzSecond - 60)
						info.stars.push(badStarInfo)
					} else {
						starInfo.seconds.push(fuzzSecond)
					}
				}
			}
			info.stars.push(starInfo)
		}
	}

	// pick some random weather types to throw in
	for (let hour = 0; hour < 24; hour++) {
		if (rng.rollMax(100) < 10) {
			const weather = forecast.weather[hour]
			if (rng.rollMax(100) < 60) {
				// pick ambiguous weather
				const options = ambiguousWeatherTypes[weather]
				const amb = options[rng.rollMax(options.length)]
				info.types.push({time: hour, type: amb})
			} else {
				// pick exact weather
				info.types.push({time: hour, type: weather})
			}
		}
	}

	return info
}

enum OutcomeKind { Success, Failure, TooManyResults }
interface Outcome {
	kind: OutcomeKind,
	results: number[]
}

const checkRange = 0xFFFFF

function performTests(weatherSeed: number, testSeed: number, hemisphere: Hemisphere, year: number, month: number): Outcome {
	const rng = Random.withSeed(testSeed)
	const guessData = GuessData.new(hemisphere)

	const monthForecast = new MonthForecast(hemisphere, weatherSeed, year, month)

	for (const dayForecast of monthForecast.days) {
		if (rng.rollMax(100) < 25 && dayForecast.specialDay == SpecialDay.None) {
			const dayInfo = makeTestDay(rng, dayForecast)
			populateGuessData(hemisphere, guessData, dayInfo)
		}
	}

	const guessBase = weatherSeed & (checkRange ^ 0x7FFFFFFF)
	const guesser = Guesser.new(guessBase, guessBase + checkRange)
	const finished = guesser.work(guessData, checkRange + 1)
	let outcome: Outcome

	if (finished == GuesserResult.Complete) {
		const resultCount = guesser.getResultCount()
		const results = []
		for (let i = 0; i < resultCount; i++) {
			results.push(guesser.getResult(i))
		}

		let kind = results.includes(weatherSeed) ? OutcomeKind.Success : OutcomeKind.Failure
		outcome = {kind, results}
	} else if (finished == GuesserResult.Failed) {
		outcome = {kind: OutcomeKind.TooManyResults, results: []}
	} else {
		// should never really happen, i think
		outcome = {kind: OutcomeKind.Failure, results: []}
	}

	guesser.free()
	guessData.free()
	rng.free()

	return outcome
}


const args = process.argv.slice(2)
if (args.length > 0) {
	const weatherSeed = parseInt(args[0], 10)
	const testSeed = parseInt(args[1], 10)
	const hemisphere = (args[2].toUpperCase().startsWith('N') || args[2] == '0') ? Hemisphere.Northern : Hemisphere.Southern
	const year = parseInt(args[3], 10)
	const month = parseInt(args[4], 10)
	const outcome = performTests(weatherSeed, testSeed, hemisphere, year, month)
	console.log(outcome)
} else {
	const weatherSeeds = [
		394476790,
		1856402561, 1522270392, 342856716, 2103512880,
		816812158, 1446278624, 1926103194, 503826528
	]
	for (const weatherSeed of weatherSeeds) {
		const testSeedMin = weatherSeed >>> 4
		const testSeedMax = testSeedMin + 20
		for (const hemisphere of [Hemisphere.Northern, Hemisphere.Southern]) {
			console.log(`Testing ${weatherSeed} in hemisphere ${hemisphere}...`)
			for (let testSeed = testSeedMin; testSeed < testSeedMax; testSeed++) {
				for (let month = 1; month <= 12; month++) {
					const year = 2020
					try {
						const outcome = performTests(weatherSeed, testSeed, hemisphere, year, month)
						if (outcome.kind == OutcomeKind.Failure) {
							console.error(`FAILED: ${weatherSeed} ${testSeed} ${hemisphere} ${year} ${month} (${outcome.results.length} results)`)
						}
					} catch (ex) {
						console.log(`weatherSeed=${weatherSeed} testSeed=${testSeed} hemisphere=${hemisphere} ${year}-${month}`)
						throw ex
					}
				}
			}
		}
	}
}


