<template>
	<b-card :title='title'>
		<b-card-text>
		</b-card-text>
		<b-card-text class='mb-0' v-for='(text, index) in broadForecast' :key='index'>
			{{ text }}
		</b-card-text>
		<b-card-text v-if='day.heavyShower'>
			🌠 Heavy meteor shower: expect shooting stars between {start} and {end}
		</b-card-text>
		<b-card-text class='small text-muted'>
			<b-button size='sm' variant='outline-secondary' class='float-right' @click='seeMore'>{{ $t('oSeeMore') }}</b-button>
			{{ $d(day.date, 'long') }}
			<br>
			{{ $t('oPattern', {pattern: day.patternName}) }}
		</b-card-text>
	</b-card>
</template>
<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator'
import { Forecast, DayForecast } from '../model'
import { Weather, SnowLevel } from '../../pkg'
import { TranslateResult } from 'vue-i18n'

enum BroadForecastType {
	Sunny, Cloudy, Rainy
}

const morningHours = [5,6,7,8,9,10,11]
const afternoonHours = [12,13,14,15,16,17]
const eveningHours = [18,19,20,21,22,23]
const nightHours = [0,1,2,3,4]

function judgeRegion(fc: DayForecast, hours: number[]): BroadForecastType {
	let clearScore = 0
	let cloudyScore = 0
	let rainScore = 0
	for (const hour of hours) {
		switch (fc.weather[hour]) {
			case Weather.Clear: clearScore += 1; break
			case Weather.Sunny: clearScore += 0.5; break
			case Weather.Cloudy: cloudyScore += 1; break
			case Weather.RainClouds: cloudyScore += 1; break
			case Weather.Rain: rainScore += 1; break
			case Weather.HeavyRain: rainScore += 2; break
		}
	}

	if (rainScore >= 1)
		return BroadForecastType.Rainy
	else if (cloudyScore >= clearScore)
		return BroadForecastType.Cloudy
	else
		return BroadForecastType.Sunny
}


interface ForecastGroup {
	type: BroadForecastType, zones: TranslateResult[]
}

@Component
export default class OverviewDayCard extends Vue {
	@Prop(String) readonly title!: string
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(DayForecast) readonly day!: DayForecast

	get combineEveningAndNight(): boolean {
		return (this.$t('oEvening') == this.$t('oNight'))
	}
	get maxTimeZones(): number {
		return this.combineEveningAndNight ? 3 : 4
	}

	collectTimeZones(morning: BroadForecastType, afternoon: BroadForecastType, evening: BroadForecastType, night: BroadForecastType): ForecastGroup[] {
		const sunny: TranslateResult[] = []
		const cloudy: TranslateResult[] = []
		const rainy: TranslateResult[] = []
		const lookup = {
			[BroadForecastType.Sunny]: sunny,
			[BroadForecastType.Cloudy]: cloudy,
			[BroadForecastType.Rainy]: rainy
		}
		lookup[morning].push(this.$t('oMorning'))
		lookup[afternoon].push(this.$t('oAfternoon'))
		lookup[evening].push(this.$t('oEvening'))
		if (!this.combineEveningAndNight || (evening != night))
			lookup[night].push(this.$t('oNight'))

		const results = []
		if (sunny.length > 0) results.push({type: BroadForecastType.Sunny, zones: sunny})
		if (cloudy.length > 0) results.push({type: BroadForecastType.Cloudy, zones: cloudy})
		if (rainy.length > 0) results.push({type: BroadForecastType.Rainy, zones: rainy})
		return results
	}

	combineTimeZones(zones: TranslateResult[], restOfDay?: boolean): TranslateResult {
		if (zones.length == this.maxTimeZones)
			return this.$t('oZoneAllDay')
		else if (restOfDay)
			return this.$t('oZoneRest')
		else if (zones.length >= 1 && zones.length <= 3)
			return this.$t('oZoneList' + zones.length, zones)
		else
			return ''
	}

	getBroadWeatherDescription(type: BroadForecastType, list: TranslateResult): string {
		switch (type) {
			case BroadForecastType.Sunny:
				return '🌤 ' + this.$t('oSunny', {list})
			case BroadForecastType.Cloudy:
				return '☁ ' + this.$t('oCloudy', {list})
			case BroadForecastType.Rainy:
				if (this.day.snowLevel == SnowLevel.None)
					return '🌧 ' + this.$t('oRain', {list})
				else
					return '🌨 ' + this.$t('oSnow', {list})
		}
	}

	get broadForecast(): string[] {
		const morning = judgeRegion(this.day, morningHours)
		const afternoon = judgeRegion(this.day, afternoonHours)
		const evening = judgeRegion(this.day, eveningHours)
		const night = judgeRegion(this.day, nightHours)

		const groups = this.collectTimeZones(morning, afternoon, evening, night)
		// sort from least zones to most zones
		groups.sort((a, b) => {
			if (a.zones.length < b.zones.length)
				return -1
			else if (a.zones.length > b.zones.length)
				return 1
			else
				return 0
		})

		const bits = []
		const zonesSeen: TranslateResult[] = []
		for (const group of groups) {
			let restOfDay = false
			if ((zonesSeen.length + group.zones.length) == this.maxTimeZones) {
				if (group.zones.every(z => !zonesSeen.includes(z))) {
					restOfDay = true
				}
			}

			const list = this.combineTimeZones(group.zones, restOfDay)
			bits.push(this.getBroadWeatherDescription(group.type, list))
			if (!restOfDay) {
				for (const zone of group.zones) {
					if (!zonesSeen.includes(zone))
						zonesSeen.push(zone)
				}
			}
		}

		return bits
	}

	combineTimes(times: Date[]): TranslateResult {
		const strs = times.map(d => this.$d(d, 'timeHM'))
		if (strs.length == 0) {
			return ''
		} else if (strs.length == 1) {
			return this.$t('oTimeList1', strs)
		} else {
			const firstPart = strs.slice(0, strs.length - 1).map(n => this.$t('oTimeList1', [n]))
			return this.$t('oTimeList2', [firstPart.join(', '), strs[strs.length - 1]])
		}
	}


	seeMore() { this.$emit('show-day', this.day) }
}
</script>

