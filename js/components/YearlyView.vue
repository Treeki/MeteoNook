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

		<b-row v-for='(row, rowNum) in monthBlocks' :key='rowNum'>
			<b-col md v-for='(month, colNum) in row' :key='colNum'>
				<div class='monthBlock border mt-3 p-2 rounded' @click='selectMonth(month)'>
					<span class='font-weight-bold'>
						{{ $t('lstMonths' + (month.month - 1)) }} {{ month.year }}
					</span>
					<div class='monthData'>
						✨ {{ $tc('yLight', month.lightShowerCount) }}
						<br/>
						🌠 {{ $tc('yHeavy', month.heavyShowerCount) }}
						<template v-if='month.rainbowCount > 0'>
							<br/>
							🌈 {{ $tc('yRainbow', month.rainbowCount) }}
							{{ (month.doubleRainbowCount > 0) ? $tc('yRainbowDouble', month.doubleRainbowCount) : '' }}
						</template>
						<template v-if='month.auroraCount > 0'>
							<br/>
							🌌 {{ $tc('yAurora' + forecast.hemiSuffix, month.auroraCount) }}
						</template>
					</div>
				</div>
			</b-col>
		</b-row>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import {Forecast, MonthForecast} from '../model'
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

	get monthBlocks(): MonthForecast[][] {
		const rows = []
		for (let y = 0; y < 4; y++) {
			const row = []
			for (let x = 0; x < 3; x++) {
				row.push(this.forecast.monthForecasts[y * 3 + x])
			}
			rows.push(row)
		}
		return rows
	}

	selectMonth(monthFC: MonthForecast) {
		this.forecast.month = monthFC.month
		this.$emit('switch-to-monthly')
	}
}
</script>
