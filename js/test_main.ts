import { Forecast, Hemisphere, MonthForecast, DayForecast, DayInfo, createDayInfo, populateGuessData, DayType, ShowerType } from './model'
import { Guesser, GuessData, Random, SpecialDay } from '../pkg'

function makeTestDay(rng: Random, forecast: DayForecast): DayInfo {
	const info = createDayInfo(forecast.date)
	if (forecast.heavyShower) {
		info.dayType = DayType.Shower
		info.showerType = ShowerType.Heavy
	}
	if (forecast.lightShower) {
		info.dayType = DayType.Shower
	}
	for (const star of forecast.shootingStars) {
		if (rng.rollMax(100) < 25) {
			info.stars.push(star)
		}
	}
	return info
}

const checkRange = 0xFFFFF

function testSeed(weatherSeed: number, testSeed: number) {
	const hemisphere = Hemisphere.Northern
	const rng = Random.withSeed(testSeed)
	const guessData = GuessData.new(hemisphere)

	const month = new MonthForecast(hemisphere, weatherSeed, 2020, 5)

	for (const dayForecast of month.days) {
		if (rng.rollMax(100) < 25 && dayForecast.specialDay == SpecialDay.None) {
			const dayInfo = makeTestDay(rng, dayForecast)
			populateGuessData(hemisphere, guessData, dayInfo)
		}
	}

	const guessBase = weatherSeed & (checkRange ^ 0x7FFFFFFF)
	const guesser = Guesser.new(guessBase, guessBase + checkRange)
	guesser.work(guessData, checkRange + 1)

	const resultCount = guesser.getResultCount()
	let found = false
	for (let i = 0; i < resultCount; i++) {
		const res = guesser.getResult(i)
		console.log(`result ${i}: ${res}`)
		if (res == weatherSeed)
			found = true
	}

	if (found)
		console.log(`success with ${resultCount - 1} false`)
	else
		console.log(`failed with ${resultCount} false`)

	guesser.free()
	guessData.free()
	rng.free()
}

testSeed(1856402561, 20)

