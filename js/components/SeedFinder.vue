<template>
	<div>
		<p class='mt-3'>{{ $t('sIntro') }}</p>
		<h4 class='mt-4'>{{ $t('sTTypes') }}</h4>
		<div v-html="$t('sTypes1')"></div>
		<p>(Image goes here)</p>
		<div v-html="$t('sTypes2')"></div>
		<h4 class='mt-4'>{{ $t('sTStars') }}</h4>
		<div v-html="$t('sStars')"></div>
		<h4 class='mt-4'>{{ $t('sTTips') }}</h4>
		<div v-html="$t('sTips')"></div>

		<h4 class='mt-4'>{{ $t('tStep1') }}</h4>
		<b-form-radio-group stacked id='seedFinderHemisphere' v-model='hemisphere'>
			<b-form-radio value='0'>{{ $t('tNorthernHemi') }}</b-form-radio>
			<b-form-radio value='1'>{{ $t('tSouthernHemi') }}</b-form-radio>
		</b-form-radio-group>

		<p>
			{{ $t('tResetBlurb') }}
			<b-button size='sm' variant='outline-secondary' @click='resetData'>
				{{ $t('tResetButton') }}
			</b-button>
		</p>

		<h4 class='mt-4'>{{ $t('tStep2') }}</h4>
		<div class='d-sm-flex'>
			<b-calendar class='align-self-start mr-3' locale='en-US' v-model='selectedDay' value-as-date></b-calendar>
			<day-editor class='flex-grow-1' :day='getDay(selectedDay)' :hemisphere='hemisphere'>
			</day-editor>
		</div>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import DayEditor from './DayEditor.vue'
import {createDayInfo, DayInfo, DayType, ShowerType, Hemisphere} from '../model'

@Component({components: {DayEditor}})
export default class SeedFinder extends Vue {
	selectedDay = new Date()
	days: {[key: string]: DayInfo} = {}
	hemisphere = Hemisphere.Northern

	getDay(date: Date) {
		const key = `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()}`
		if (this.days[key] === undefined)
			Vue.set(this.days, key, createDayInfo(date))
		return this.days[key]
	}
	
	resetData() {
		this.days = {}
	}
}
</script>
