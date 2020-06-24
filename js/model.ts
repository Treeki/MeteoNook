export enum DayType { NoData = 0, None, Shower, Rainbow, Aurora }
export enum ShowerType { NotSure = 0, Light, Heavy }

import {Hemisphere, Weather, SpecialDay, getMonthLength, Pattern, getPattern, getWeather, getWindPower, isSpecialDay, SnowLevel, CloudLevel, FogLevel, getSnowLevel, getCloudLevel, getFogLevel, checkWaterFog, RainbowInfo, getRainbowInfo, isAuroraPattern, fromLinearHour, toLinearHour, canHaveShootingStars, queryStars, getStarSecond} from '../pkg'
export {Hemisphere, Weather, SpecialDay, getMonthLength}

export interface WeatherTypeInfo {
	time: number, type: Weather
}
export interface StarInfo {
	hour: number, minute: number, seconds: number[]
}
export interface GapInfo {
	startHour: number, startMinute: number, endHour: number, endMinute: number
}

export interface DayInfo {
	y: number,
	m: number,
	d: number,
	dayType: DayType,
	showerType: ShowerType,
	rainbowTime: number,
	rainbowDouble: boolean,
	auroraFine01: boolean,
	auroraFine03: boolean,
	auroraFine05: boolean,
	types: WeatherTypeInfo[],
	stars: StarInfo[],
	gaps: GapInfo[]
}

export function createDayInfo(date: Date): DayInfo {
	return {
		y: date.getFullYear(), m: date.getMonth() + 1, d: date.getDate(),
		dayType: DayType.NoData, showerType: ShowerType.NotSure,
		rainbowTime: 10, rainbowDouble: false,
		auroraFine01: false, auroraFine03: false, auroraFine05: false,
		types: [], stars: [], gaps: []
	}
}

export function isDayNonEmpty(day: DayInfo): boolean {
	return (day.dayType != DayType.NoData || day.types.length > 0)
}


export class Forecast {
	hemisphere: Hemisphere
	seed: number
	year: number
	month: number
	monthForecasts: MonthForecast[]

	constructor() {
		const now = new Date()
		this.hemisphere = Hemisphere.Northern
		this.seed = 1856402561
		this.year = now.getFullYear()
		this.month = now.getMonth() + 1
		this.monthForecasts = []
		this.regenerateForecasts()
	}

	setPreviousYear() {
		this.year -= 1
		this.regenerateForecasts()
	}
	setNextYear() {
		this.year += 1
		this.regenerateForecasts()
	}
	setPreviousMonth() {
		this.month -= 1
		if (this.month <= 0) {
			this.month = 12
			this.year -= 1
		}
		this.regenerateForecasts()
	}
	setNextMonth() {
		this.month += 1
		if (this.month >= 13) {
			this.month = 1
			this.year += 1
		}
		this.regenerateForecasts()
	}

	regenerateForecasts() {
		this.monthForecasts.splice(0, this.monthForecasts.length)
		for (let month = 1; month <= 12; month++) {
			const fc = new MonthForecast(this.hemisphere, this.seed, this.year, month)
			this.monthForecasts.push(fc)
		}
	}

	get currentMonth(): MonthForecast {
		return this.monthForecasts[this.month - 1]
	}
}


export class MonthForecast {
	readonly days: DayForecast[]

	constructor(
		readonly hemisphere: Hemisphere,
		readonly seed: number,
		readonly year: number,
		readonly month: number
	) {
		const dayCount = getMonthLength(year, month)
		this.days = []
		for (let day = 1; day <= dayCount; day++) {
			this.days.push(new DayForecast(hemisphere, seed, year, month, day))
		}
	}
}

export class DayForecast {
	readonly pattern: Pattern
	readonly weather: Weather[]
	readonly windPower: number[]
	readonly specialDay: SpecialDay
	readonly snowLevel: SnowLevel
	readonly cloudLevel: CloudLevel
	readonly fogLevel: FogLevel
	readonly waterFog: boolean
	readonly rainbow: RainbowInfo
	readonly aurora: boolean
	readonly shootingStars: StarInfo[]

	constructor(
		readonly hemisphere: Hemisphere,
		readonly seed: number,
		readonly year: number,
		readonly month: number,
		readonly day: number
	) {
		// collect data from the library
		this.pattern = getPattern(hemisphere, seed, year, month, day)
		this.specialDay = isSpecialDay(hemisphere, year, month, day)
		this.snowLevel = getSnowLevel(hemisphere, month, day)
		this.cloudLevel = getCloudLevel(hemisphere, month, day)
		this.fogLevel = getFogLevel(hemisphere, month, day)
		this.waterFog = (this.fogLevel != FogLevel.None) && checkWaterFog(seed, year, month, day)
		this.rainbow = getRainbowInfo(hemisphere, seed, year, month, day, this.pattern)
		this.aurora = isAuroraPattern(hemisphere, month, day, this.pattern)

		this.weather = []
		this.windPower = []
		for (let hour = 0; hour < 24; hour++) {
			this.weather.push(getWeather(hour, this.pattern))
			this.windPower.push(getWindPower(seed, year, month, day, hour, this.pattern))
		}

		this.shootingStars = []
		for (let linearHour = 0; linearHour < 9; linearHour++) {
			const hour = fromLinearHour(linearHour)
			if (canHaveShootingStars(hour, this.pattern)) {
				for (let minute = 0; minute < 60; minute++) {
					const starCount = queryStars(seed, year, month, day, hour, minute, this.pattern)
					if (starCount > 0) {
						const star: StarInfo = {hour, minute, seconds: []}
						for (let i = 0; i < starCount; i++) {
							star.seconds.push(getStarSecond(i))
						}
						this.shootingStars.push(star)
					}
				}
			}
		}
	}
}
