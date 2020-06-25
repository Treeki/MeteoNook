<template>
	<div>
		<p class='mt-3'>{{ $t('sIntro') }}</p>
		<h4 class='mt-4'>{{ $t('sTTypes') }}</h4>
		<div v-html="timeify($t('sTypes1'))"></div>
		<p>(Image goes here)</p>
		<div v-html="timeify($t('sTypes2'))"></div>
		<h4 class='mt-4'>{{ $t('sTStars') }}</h4>
		<div v-html="timeify($t('sStars'))"></div>
		<h4 class='mt-4'>{{ $t('sTTips') }}</h4>
		<div v-html="timeify($t('sTips'))"></div>

		<h4 class='mt-4'>{{ $t('tStep1') }}</h4>
		<b-form-radio-group class='mb-3' stacked id='seedFinderHemisphere' v-model.number='hemisphere'>
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
			<b-calendar
				class='align-self-start mr-3'
				min='2000-01-01' max='2060-12-31'
				label-help=''
				:locale='$i18n.locale'
				v-model='selectedDay'
				:date-info-fn='dateClass'
				value-as-date
				></b-calendar>
			<day-editor class='flex-grow-1' :day='getDay(selectedDay)' :hemisphere='hemisphere'>
			</day-editor>
		</div>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import DayEditor from './DayEditor.vue'
import {createDayInfo, DayInfo, DayType, ShowerType, Hemisphere, isDayNonEmpty} from '../model'

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

	timeify(str: string): string {
		// accessing time12 here forces this component to be
		// updated whenever time12 changes...
		const awfulHack = (this.$root as any).time12

		return str.replace(/[012]\d:\d\d:\d\d/g, time => {
			const [hS, mS, sS] = time.split(':')
			const h = parseInt(hS, 10), m = parseInt(mS, 10), s = parseInt(sS, 10)
			return this.$d(new Date(0, 0, 0, h, m, s), 'timeHMS')
		}).replace(/[012]\d:\d\d/g, time => {
			const [hS, mS] = time.split(':')
			const h = parseInt(hS, 10), m = parseInt(mS, 10)
			return this.$d(new Date(0, 0, 0, h, m), 'timeHM')
		}).replace(/[012]\dH/g, time => {
			const h = parseInt(time.slice(0, 2), 10)
			return this.$d(new Date(0, 0, 0, h), 'timeH')
		})
	}

	dateClass(ymd: string, date: Date): string {
		const key = `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()}`
		const day = this.days[key]
		if (day !== undefined && isDayNonEmpty(day))
			return 'table-info'
		return ''
	}
}
</script>
