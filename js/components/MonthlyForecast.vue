<style>
.monthBlock { cursor: pointer; }
</style>

<template>
	<div>
		<div class='d-flex border-bottom pb-2'>
			<b-button variant='outline-secondary' size='sm' :disabled='prevDisabled' @click='forecast.setPreviousMonth()'>
				&laquo; {{ this.previousMonth }}
			</b-button>
			<span class='flex-grow-1 font-weight-bold text-center'>
				{{ this.forecast.month }} {{ this.forecast.year }}
			</span>
			<b-button variant='outline-secondary' size='sm' :disabled='nextDisabled' @click='forecast.setNextMonth()'>
				{{ this.nextMonth }} &raquo;
			</b-button>
		</div>

		<b-row class='my-2' v-for='day in dayCount' :key='day' @click='dayClicked(day)'>
			<b-col sm='4'>
				{{ day }}. PatName
			</b-col>
			<b-col sm='8'>
				PatStuff
			</b-col>
		</b-row>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import {Forecast} from '../model'
import {isSpecialDay, getMonthLength} from '../../pkg'

const MonthlyForecastProps = Vue.extend({
	props: {
		forecast: Forecast
	}
})

@Component
export default class MonthlyForecast extends MonthlyForecastProps {
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

	dayClicked(day: number) {
		this.$emit('show-day', this.forecast.year, this.forecast.month, day)
	}
}
</script>

