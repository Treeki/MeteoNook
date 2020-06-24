export enum DayType { NoData = 0, None, Shower, Rainbow, Aurora }
export enum ShowerType { NotSure = 0, Light, Heavy }

import {Hemisphere, Weather, SpecialDay, getMonthLength} from '../pkg'
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
	seed: number
	hemisphere: Hemisphere
	year: number
	month: number

	constructor() {
		const now = new Date()
		this.seed = 1856402561
		this.hemisphere = Hemisphere.Northern
		this.year = now.getFullYear()
		this.month = now.getMonth() + 1
	}

	setPreviousYear() {
		this.year -= 1
	}
	setNextYear() {
		this.year += 1
	}
	setPreviousMonth() {
		this.month -= 1
		if (this.month <= 0) {
			this.month = 12
			this.year -= 1
		}
	}
	setNextMonth() {
		this.month += 1
		if (this.month >= 13) {
			this.month = 1
			this.year += 1
		}
	}
}
