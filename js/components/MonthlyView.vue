<style>
	.monthBlock { cursor: pointer; }

	.heavyShower { background-color: goldenrod; border-radius: 4px; }
	.lightShower { background-color: yellow; border-radius: 4px; }

	.wday-0 { border-bottom: 1px solid #888; }
	.wday-0, .wday-6 { background: #eee; }

	.mWeekday { width: 2.5em; display: inline-block; text-align: right; font-style: italic; }
	.wday-0 .mWeekday, .wday-6 .mWeekday {
		font-style: normal;
		font-weight: bold;
		color: darkblue;
	}
</style>

<template>
	<div>
		<div class='d-flex border-bottom pb-2'>
			<b-button variant='outline-secondary' size='sm' :disabled='prevDisabled' @click='forecast.setPreviousMonth()'>
				&laquo; {{ $t('lstMonths' + (this.previousMonth - 1)) }}
			</b-button>
			<span class='flex-grow-1 font-weight-bold text-center'>
				{{ $t('lstMonths' + (this.forecast.month - 1)) }} {{ this.forecast.year }}
			</span>
			<b-button variant='outline-secondary' size='sm' :disabled='nextDisabled' @click='forecast.setNextMonth()'>
				{{ $t('lstMonths' + (this.nextMonth - 1)) }} &raquo;
			</b-button>
		</div>

		<b-row :class="'my-2 wday-' + day.weekday" v-for='day in month.days' :key='day.day' @click='dayClicked(day)'>
			<b-col sm='4' :class='{heavyShower: day.heavyShower, lightShower: day.lightShower}'>
				<span class='mWeekday'>{{ $t('lstWeekdays' + day.weekday) }}</span>
				{{ day.day }}.
				{{ getDayPrefix(day) }}
				{{ day.patternName }}
			</b-col>
			<b-col sm='8'>
				<day-visualisation-bar :forecast='forecast' :day='day'></day-visualisation-bar>
			</b-col>
		</b-row>
	</div>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import {Forecast, MonthForecast, DayForecast} from '../model'
import {isSpecialDay, getMonthLength} from '../../pkg'
import DayVisualisationBar from './DayVisualisationBar.vue'

@Component({components: {DayVisualisationBar}})
export default class MonthlyView extends Vue {
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(MonthForecast) readonly month!: MonthForecast

	get prevDisabled() {
		return (this.forecast.year <= 2000) && (this.forecast.month == 1)
	}
	get nextDisabled() {
		return (this.forecast.year >= 2060) && (this.forecast.month == 12)
	}
	get previousMonth() {
		return (this.forecast.month == 1) ? 12 : (this.forecast.month - 1)
	}
	get nextMonth() {
		return (this.forecast.month == 12) ? 1 : (this.forecast.month + 1)
	}

	get dayCount() {
		return getMonthLength(this.forecast.year, this.forecast.month)
	}

	getDayPrefix(day: DayForecast): string {
		const bits = []
		if (day.heavyShower) bits.push('🌠')
		if (day.lightShower) bits.push('✨')
		if (day.rainbowCount == 1) bits.push('🌈')
		if (day.rainbowCount == 2) bits.push('🌈🌈')
		if (day.aurora) bits.push('🌌')
		if (day.heavyFog || day.waterFog) bits.push('🌫')
		return bits.join(' ')
	}

	dayClicked(day: DayForecast) {
		this.$emit('show-day', day)
	}
}
</script>
