<style>
.monthBlock { cursor: pointer; }
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

		<b-row class='my-2' v-for='day in month.days' :key='day.day' @click='dayClicked(day)'>
			<b-col sm='4'>
				{{ $t('lstWeekdays' + day.date.getDay()) }}
				{{ day.day }}.
				{{ day.patternName }}
			</b-col>
			<b-col sm='8'>
				PatStuff
			</b-col>
		</b-row>
	</div>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import {Forecast, MonthForecast, DayForecast} from '../model'
import {isSpecialDay, getMonthLength} from '../../pkg'

@Component
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

	dayClicked(day: DayForecast) {
		this.$emit('show-day', day)
	}
}
</script>

