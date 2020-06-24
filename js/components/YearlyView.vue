<style>
.monthBlock { cursor: pointer; }
</style>

<template>
	<div>
		<div class='d-flex border-bottom pb-2'>
			<b-button variant='outline-secondary' size='sm' :disabled='prevDisabled' @click='forecast.setPreviousYear()'>
				&laquo; {{ this.previousYear }}
			</b-button>
			<span class='flex-grow-1 font-weight-bold text-center'>
				{{ this.forecast.year }}
			</span>
			<b-button variant='outline-secondary' size='sm' :disabled='nextDisabled' @click='forecast.setNextYear()'>
				{{ this.nextYear }} &raquo;
			</b-button>
		</div>

		<b-row v-for='i in 4' :key='i'>
			<b-col md v-for='j in 3' :key='j'>
				<div class='monthBlock border mt-3 p-2 rounded' @click='selectMonth(i,j)'>
					<span class='font-weight-bold'>Month</span>
					<div class='monthData'>
						Data
					</div>
				</div>
			</b-col>
		</b-row>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import {Forecast} from '../model'
import {isSpecialDay} from '../../pkg'

const YearlyViewProps = Vue.extend({
	props: {
		forecast: Forecast
	}
})

@Component
export default class YearlyView extends YearlyViewProps {
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

	selectMonth(i: number, j: number) {
		console.log(`clique: ${i} . ${j}`)
		const month = ((i - 1) * 3) + j
		this.forecast.month = month
		this.$emit('switch-to-monthly')
	}
}
</script>
