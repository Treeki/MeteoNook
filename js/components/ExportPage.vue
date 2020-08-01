<template>
	<div>
		<h2 class='mt-3'>{{ $t('hTExport') }}</h2>
		<div v-html="$t('hExport')"></div>
		<h4 class='mt-3'>{{ $t('hICalExport') }}</h4>
		<div v-html="$t('tICalExport')"></div>
		<h4 class='mt-3'>{{ $t('hICalOptions') }}</h4>
		<div>
			<b-checkbox v-model='heavyShower'>Heavy Showers</b-checkbox>
			<b-checkbox v-model='lightShower'>Light Showers</b-checkbox>
			<b-checkbox v-model='rainbow'>{{ $t('tRainbow') }}</b-checkbox>
			<b-checkbox v-model='aurora'>{{ $t('tAurora' + this.forecast.hemiSuffix) }}</b-checkbox>
		</div>
		<div>
			<b-button variant='outline-secondary' size='sm' :disabled='prevDisabled' @click='forecast.setPreviousYear()'>
				&laquo; {{ this.previousYear }}
			</b-button>
			<b-button size='sm' @click='exportCalendar'>
				{{ $t('tICalExportButton', [this.forecast.year]) }}
			</b-button>
			<b-button variant='outline-secondary' size='sm' :disabled='nextDisabled' @click='forecast.setNextYear()'>
				{{ this.nextYear }} &raquo;
			</b-button>
		</div>
	</div>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import {isSpecialDay, getMonthLength} from '../../pkg'
import {DayForecast, Forecast, Hemisphere, MonthForecast, StarInfo} from '../model'
import { makeTime } from '../utils'

@Component
export default class ExportPage extends Vue {
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(Number) readonly hemisphere!: Hemisphere
	heavyShower = true
	lightShower = true
	rainbow = false
	aurora = false

	get prevDisabled() {
		return this.forecast.year <= 2000
	}
	get nextDisabled() {
		return this.forecast.year >= 2060
	}
	get previousYear() {
		return this.forecast.year - 1
	}
	get nextYear() {
		return this.forecast.year + 1
	}

	exportCalendar() {
		this.addEvents(this.forecast)
		return this.$ics.download(this.forecast.year + 'calendar')
	}

	addEvents(forecast: Forecast) {
		for (const month of forecast.monthForecasts) {
			for (const day of month.days) {
				this.addEventsPerDay(day)
			}
		}
	}

	addEventsPerDay(day: DayForecast) {
		if (this.shouldAddEvent(day))
			this.$ics.addEvent(this.$i18n.locale, this.getTitle(day), this.getDescription(day), '', day.date, day.date)
	}

	shouldAddEvent(day: DayForecast): boolean {
		return ((this.heavyShower && day.heavyShower) ||
			(this.lightShower && day.lightShower) ||
			(this.aurora && day.aurora) ||
			(this.rainbow && day.rainbowCount > 0))
	}

	getTitle(day: DayForecast): string {
		const bits = []
		if (this.heavyShower && day.heavyShower) bits.push('🌠' + this.$tc('oHeavyShower', 1))
		if (this.lightShower && day.lightShower) bits.push('✨' + this.$tc('oLightShower', 1))
		if (this.rainbow && day.rainbowCount == 1) bits.push('🌈' + this.$tc('dRainbowSingle', 1))
		if (this.rainbow && day.rainbowCount == 2) bits.push('🌈🌈' + this.$tc('dRainbowDouble', 1))
		if (this.aurora && day.aurora) bits.push('🌌' +  + this.$tc('dAurora' + this.forecast.hemiSuffix, 1))
		bits.push('(' + this.$tc('appTitle', 1) + ')')
		return bits.join(' ')
	}
	getDescription(day: DayForecast): string {
		return day.patternName
	}
}
</script>

