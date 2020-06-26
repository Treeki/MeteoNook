<template>
	<b-modal :id='id' title='Day' scrollable hide-footer>
		Shit!!!
	</b-modal>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import {DayInfo, DayType, Forecast, DayForecast} from '../model'

@Component
export default class DayModal extends Vue {
	@Prop(String) readonly id!: string
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(DayForecast) readonly day!: DayForecast
}
</script>

