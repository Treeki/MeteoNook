<template>
	<b-modal :id='id' title='Day' scrollable hide-footer>
		Shit!!!
	</b-modal>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import {DayInfo, DayType, Forecast} from '../model'

const DayModalProps = Vue.extend({
	props: {
		id: String,
		forecast: Forecast,
		date: Date
	}
})

@Component
export default class DayModal extends DayModalProps {
}
</script>

