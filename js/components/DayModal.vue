<style>
	.hour>span { margin-left: 0.25em; }
</style>

<template>
	<b-modal :id='id' :title='modalTitle' scrollable hide-footer>
		<p v-show='specialDayWarning'>🎉 {{ specialDayWarning }}</p>

		<template v-for='hour in hours'>
			<p class='hour' :key='hour'>
				<b>{{ $d(hourDates[hour], 'timeHM') }}</b>:
				<span :title="$t('dWindTooltip')" v-if='!day.patternPreviewMode'>
					🍃{{ day.windPower[hour] }}
				</span>
				<span :title="$t('dWindTooltip')" v-if='day.patternPreviewMode'>
					🍃{{ day.windPowerMin[hour] }}-{{ day.windPowerMax[hour] }}
				</span>
				{{ $t(weatherNameStringPrefix + day.weather[hour]) }}
				<span v-if='hour == 7 && day.heavyFog'>
					🌫 {{ $t('dHeavyFog') }}
				</span>
				<span v-if='hour == 7 && day.waterFog'>
					🌫 {{ $t('dWaterFog') }}
				</span>
				<span v-if='!day.patternPreviewMode && hour == day.rainbowHour && day.rainbowCount == 1'>
					🌈 {{ $t('dRainbowSingle') }}
				</span>
				<span v-if='!day.patternPreviewMode && hour == day.rainbowHour && day.rainbowCount == 2'>
					🌈🌈 {{ $t('dRainbowDouble') }}
				</span>
				<span v-if='day.patternPreviewMode && hour == day.rainbowHour && day.rainbowCount == 1'>
					🌈 {{ $t('dRainbowEither') }}
				</span>
				<span v-if='day.hasAuroraAtHour(hour)'>
					🌌 {{ $t('dAurora' + forecast.hemiSuffix) }}
				</span>
				<span v-if='day.patternPreviewMode && day.hasStarsAtHour(hour)'>
					🌠 {{ $t('dStars') }}
				</span>
			</p>
			<p v-for='(times, index) in sortedStars[hour]' :key='10000 + hour * 1000 + index'>
				🌠 {{ times }}
			</p>
		</template>
	</b-modal>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import {DayInfo, DayType, Forecast, DayForecast, getPatternName, StarInfo} from '../model'
import { TranslateResult } from 'vue-i18n'
import { Pattern, SpecialDay, SnowLevel } from '../../pkg'
import { makeTime } from '../utils'

@Component
export default class DayModal extends Vue {
	@Prop(String) readonly id!: string
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(DayForecast) readonly day!: DayForecast

	get hours(): number[] {
		return [5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,0,1,2,3,4]
	}
	get hourDates(): Date[] {
		const results = []
		for (let i = 0; i < 24; i++)
			results.push(makeTime(i, 0))
		return results
	}
	get sortedStars(): {[hour: number]: String[]} {
		const y = this.day.year
		const m = this.day.month - 1
		const d = this.day.day
		const results: {[hour: number]: String[]} = {}
		for (const star of this.day.shootingStars) {
			let k = results[star.hour]
			if (k === undefined)
				k = results[star.hour] = []

			const baseDate = new Date(y, m, d, star.hour, star.minute)
			const seconds = star.seconds.filter(s => s != 99).map(s => ':' + ('0' + s).slice(-2))
			k.push(this.$d(baseDate, 'timeHM') + ' - ' + seconds.join(', '))
		}
		return results
	}

	get modalTitle(): TranslateResult {
		const pattern = getPatternName(this.day.pattern)
		if (this.day.patternPreviewMode) {
			return this.$t('dTitlePreview', {pattern})
		} else {
			const date = this.$d(this.day.date, 'long')
			return this.$t('dTitleDay', {date, pattern})
		}
	}

	get specialDayWarning(): TranslateResult {
		if (this.day.pattern == Pattern.EventDay00 && this.day.specialDay != SpecialDay.None)
			return this.$t('dSpDay', {day: this.$t('lstSpecialDays' + (this.day.specialDay - 1))})
		else
			return ''
	}

	get weatherNameStringPrefix(): string {
		if (this.day.snowLevel === SnowLevel.None)
			return 'lstPatternNoSnow'
		else
			return 'lstPatternSnow'
	}
}
</script>

