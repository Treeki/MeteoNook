<style>
	.w { height: 1.5em; padding: 0; position: relative; }
	.w:first-child { border-radius: 4px 0 0 4px; }
	.w:last-child { border-radius: 0 4px 4px 0; }
	@media (max-width: 767.98px) {
		.w { font-size: 75%; }
	}

	.w { background: linear-gradient(to bottom, #5b9bff, #8ee1ff); }
	.w:after {
		content: '';
		text-align: center;
		display: block;
		position: absolute;
		top: 0; left: 0; right: 0; bottom: 0;
	}
	.w1:after { content: '🌤'; }
	.w2:after { content: '🌥'; }

	.w3,.w4,.w5 { background: linear-gradient(to bottom, #657ca1, #bfdbe6); }
	.w3:after { content: '☁'; }
	.w4:after { content: '💧'; font-size: 50%; line-height: 3em; }
	.w5:after { content: '💦'; font-size: 75%; line-height: 2em; }
	.w4.snow:after, .w5.snow:after { content: '❄️'; }

	.boundary { border-right: 1px solid black; }
</style>

<template>
	<div class='d-flex'>
		<b-col :class="['w', 'w'+day.weather[hour], {snow: hasSnow}, {boundary: hour == 11 || hour == 23}]" v-for='hour in hours' :key='hour'>
		</b-col>
	</div>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator'
import { Forecast, DayForecast } from '../model'
import { SnowLevel } from '../../pkg'

@Component
export default class DayVisualisationBar extends Vue {
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(DayForecast) readonly day!: DayForecast

	get hasSnow(): boolean {
		return this.day.snowLevel != SnowLevel.None
	}

	get hours(): number[] {
		return [5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,0,1,2,3,4]
	}
}

</script>
