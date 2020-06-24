<template>
	<div>
		<p v-show='specialDayWarning'>{{ specialDayWarning }}</p>
		<h5>Patterns</h5>
		<b-form-radio-group stacked id='dayType' v-model='day.dayType'>
			<b-form-radio value='0'>No data</b-form-radio>
			<b-form-radio value='2'>Meteor shower</b-form-radio>
			<b-form-radio value='3'>Rainbow</b-form-radio>
			<b-form-radio value='4'>Aurora</b-form-radio>
			<b-form-radio value='1'>None</b-form-radio>
		</b-form-radio-group>

		<div v-show='isShowerMode()'>
			<h5 class='mt-3'>Shower info</h5>
			<b-form-radio-group stacked id='showerType' v-model='day.showerType'>
				<b-form-radio value='1'>Light</b-form-radio>
				<b-form-radio value='2'>Heavy</b-form-radio>
				<b-form-radio value='0'>Not sure</b-form-radio>
			</b-form-radio-group>

			<h5 class='mt-3'>Times</h5>
			<b-input-group v-for='(star, index) in day.stars' :key='"st" + index' size='sm' class='mb-1'>
				<b-form-select v-model='star.hour'>
					<b-form-select-option :value='hour' v-for='hour in [19,20,21,22,23,0,1,2,3]' :key='hour'>{{ hour }}</b-form-select-option>
				</b-form-select>
				<b-form-select v-model='star.minute'>
					<b-form-select-option :value='min - 1' v-for='min in 60' :key='min'>{{ min - 1 }}</b-form-select-option>
				</b-form-select>
				<b-input-group-append>
					<b-button variant='outline-secondary' @click='editStarSeconds(index)'>{{ describeSeconds(index) }}</b-button>
					<b-button variant='outline-danger' @click='removeStar(index)'>✖️</b-button>
				</b-input-group-append>
			</b-input-group>
			<b-button block size='sm' variant='outline-primary' @click='addStar'>Add Time</b-button>
		</div>

		<b-modal id='secondsEditor' title='Edit Seconds' ok-title='Save' scrollable @ok='saveSeconds'>
			<b-form-group :id='"star" + i' :key='i' :label='"Star " + i' label-for='time' label-cols-sm='4' v-for='i in 8'>
				<b-form-select v-model='starModalSeconds[i - 1]' id='time'>
					<b-form-select-option :value='99'>Don't know</b-form-select-option>
					<b-form-select-option :value='sec - 1' :key='sec' v-for='sec in 60'>{{ sec - 1 }}</b-form-select-option>
				</b-form-select>
			</b-form-group>
		</b-modal>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import {DayInfo, DayType} from '../model'
import {isSpecialDay} from '../../pkg'

const DayEditorProps = Vue.extend({
	props: {
		day: Object
	}
})

@Component
export default class DayEditor extends DayEditorProps {
	get specialDayWarning(): string {
		const number = isSpecialDay(0, this.day.y, this.day.m, this.day.d)
		return `Special day kind ${number}`
	}



	starModalIndex: number = 0
	starModalSeconds: number[] = []

	isShowerMode() { return this.day.dayType == DayType.Shower }

	addStar() {
		let hour = 19, minute = 0
		if (this.day.stars.length > 0) {
			const last = this.day.stars[this.day.stars.length - 1]
			hour = last.hour
			minute = last.minute + 1
			if (minute == 60) {
				if (hour == 3) {
					minute = 59
				} else {
					hour += 1
					if (hour == 24) hour = 0
					minute = 0
				}
			}
		}
		this.day.stars.push({hour: hour, minute: minute, seconds: [99,99,99,99,99,99,99,99]})
	}

	describeSeconds(index: number) {
		const seconds = []
		for (const s of this.day.stars[index].seconds) {
			if (s != 99) seconds.push(':' + ('0' + s).slice(-2))
		}
		return (seconds.length == 0) ? 'secs' : seconds.join(', ')
	}

	editStarSeconds(index: number) {
		this.starModalIndex = index
		this.starModalSeconds = this.day.stars[index].seconds.slice()
		this.$bvModal.show('secondsEditor')
	}
	saveSeconds(index: number) {
		this.day.stars[this.starModalIndex].seconds = this.starModalSeconds
	}

	removeStar(index: number) {
		this.day.stars.splice(index, 1)
	}
}
</script>
