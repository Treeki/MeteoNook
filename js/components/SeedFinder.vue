<template>
	<div>
		<div class='d-sm-flex'>
			<b-calendar class='align-self-start' locale='en-US' v-model='selectedDay' value-as-date></b-calendar>
			<day-editor class='flex-grow-1' :day='getDay(selectedDay)'>
			</day-editor>
		</div>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import DayEditor from './DayEditor.vue'
import {createDayInfo, DayInfo, DayType, ShowerType} from '../model'

@Component({components: {DayEditor}})
export default class SeedFinder extends Vue {
	selectedDay = new Date()
	days: {[key: string]: DayInfo} = {}

	getDay(date: Date) {
		const key = `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()}`
		if (this.days[key] === undefined)
			Vue.set(this.days, key, createDayInfo(date))
		return this.days[key]
	}
}

/*export default {
	name: 'seed-finder',
	components: {DayEditor},
	data() {
		return {
			selectedDay: new Date(),
			days: {}
		}
	},
	methods: {
		getDay(date) {
			const key = `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()}`
			if (this.days[key] === undefined) {
				Vue.set(this.days, key, {
					y: date.getFullYear(), m: date.getMonth() + 1, d: date.getDate(),
					dayType: DayType.NoData, showerType: ShowerType.NotSure,
					rainbowTime: 10, rainbowDouble: false,
					auroraFine01: false, auroraFine03: false, auroraFine05: false,
					types: [], stars: [], gaps: []
				})
			}
			return this.days[key]
		}
	}
}*/
</script>
