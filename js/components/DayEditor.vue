<style>
.patNameInList {
	cursor: pointer;
}
.patNameInList:not(:last-child):after {
	content: ', ';
}
</style>

<template>
	<div>
		<p v-show='specialDayWarning' v-html='specialDayWarning'></p>
		<h5>{{ $t('tTPatterns') }}</h5>
		<b-form-radio-group stacked id='dayType' v-model='day.dayType'>
			<b-form-radio value='0'>{{ $t('tNoData') }}</b-form-radio>
			<b-form-radio value='2'>{{ $t('tShower') }}</b-form-radio>
			<b-form-radio value='3' v-show='hasRainbowOption'>{{ $t('tRainbow') }}</b-form-radio>
			<b-form-radio value='4' v-show='hasAuroraOption'>{{ $t('tAurora' + ((hemisphere == 0) ? 'N' : 'S')) }}</b-form-radio>
			<b-form-radio value='1'>{{ $t('tNone') }}</b-form-radio>
		</b-form-radio-group>

		<div v-show='isShowerMode'>
			<h5 class='mt-3'>{{ $t('tTShower') }}</h5>
			<b-form-radio-group stacked id='showerType' v-model='day.showerType'>
				<b-form-radio value='2'>{{ $t('tHeavy') }}</b-form-radio>
				<b-form-radio value='1'>{{ $t('tLight') }}</b-form-radio>
				<b-form-radio value='0'>{{ $t('tNotSure') }}</b-form-radio>
			</b-form-radio-group>

			<h5 class='mt-3'>{{ $t('tTTimes') }}</h5>
			<b-input-group v-for='(star, index) in day.stars' :key='"st" + index' size='sm' class='mb-1'>
				<b-form-select v-model.number='star.hour' :options='starHourOptions'></b-form-select>
				<b-form-select v-model.number='star.minute' :options='minuteOptions'></b-form-select>
				<b-input-group-append>
					<b-button variant='outline-secondary' @click='editStarSeconds(index)'>{{ describeSeconds(index) }}</b-button>
					<b-button variant='outline-danger' @click='removeStar(index)'>✖️</b-button>
				</b-input-group-append>
			</b-input-group>
			<b-button block size='sm' variant='outline-primary' @click='addStar'>{{ $t('tAddTime') }}</b-button>

			<h5 class='mt-3'>{{ $t('tTGaps') }}</h5>
			<b-input-group v-for='(gap, index) in day.gaps' :key='"sg" + index' size='sm' class='mb-1'>
				<b-form-select v-model.number='gap.startHour' :options='starHourOptions'></b-form-select>
				<b-form-select v-model.number='gap.startMinute' :options='minuteOptions'></b-form-select>
				<b-input-group-prepend class='b-input-group-append'>
					<b-input-group-text>{{ $t('tTo') }}</b-input-group-text>
				</b-input-group-prepend>
				<b-form-select v-model.number='gap.endHour' :options='starHourOptions'></b-form-select>
				<b-form-select v-model.number='gap.endMinute' :options='minuteOptions'></b-form-select>
				<b-input-group-append>
					<b-button variant='outline-danger' @click='removeGap(index)'>✖️</b-button>
				</b-input-group-append>
			</b-input-group>
			<div class='d-flex'>
				<b-button class='flex-grow-1 mr-2' size='sm' variant='outline-primary' @click='addGap'>{{ $t('tAddGap') }}</b-button>
				<b-button class='flex-grow-1' size='sm' variant='outline-secondary' @click='makeGapsFromTimes'>{{ $t('tMakeFromTimes') }}</b-button>
			</div>
		</div>

		<div v-show='isTypesMode'>
			<h5 class='mt-3'>{{ $t('tTTypes') }}</h5>
			<b-input-group v-for='(type, index) in day.types' :key='index' size='sm' class='mb-1'>
				<b-form-select v-model.number='type.time' :options='hourOptions'></b-form-select>
				<b-form-select v-model.number='type.type' :options='weatherTypeOptions'></b-form-select>
				<b-input-group-append>
					<b-button variant='outline-danger' @click='removeType(index)'>✖️</b-button>
				</b-input-group-append>
			</b-input-group>
			<b-button block size='sm' variant='outline-primary' @click='addType'>
				{{ $t(typesListFull ? 'tEnoughTimes' : 'tAddType') }}
			</b-button>
		</div>

		<div v-show='isRainbowMode'>
			<h5 class='mt-3'>{{ $t('tTRainbow') }}</h5>
			<b-form-group label-cols-sm='3' :label="$t('tTime')" label-for='rainbowTime'>
				<b-form-select v-model.number='day.rainbowTime' :options='rainbowTimeOptions'></b-form-select>
			</b-form-group>
			<!-- there's gotta be a better way to do this... -->
			<div class='form-group row'>
				<div class='col-sm-3'></div>
				<div class='col-sm-9'>
					<b-form-checkbox v-model='day.rainbowDouble'>{{ $t('tDoubleRainbow') }} 🌈🌈</b-form-checkbox>
				</div>
			</div>
		</div>

		<div v-show='isAuroraMode'>
			<h5 class='mt-3'>{{ $t('tTAurora') }}</h5>
			<p class='mb-1'>{{ $t('tAuroraFit') }}</p>
			<b-form-checkbox v-model='day.auroraFine01'>{{ labelAuroraFine01 }}</b-form-checkbox>
			<b-form-checkbox v-model='day.auroraFine03'>{{ labelAuroraFine03 }}</b-form-checkbox>
			<b-form-checkbox v-model='day.auroraFine05'>{{ labelAuroraFine05 }}</b-form-checkbox>
		</div>

		<p class='mt-2'>
			<b>{{ $t('tPatterns') }}</b>
			<span v-show='possiblePatternNames.length == 0' v-html="$t('tPatternsNone')"></span>
			<span class='patNameInList' v-for='pat in possiblePatternNames' :key='pat' @click='demoPattern(pat)'>{{ pat }}</span>
		</p>

		<b-modal id='secondsEditor' :title='starSecondsTitle' :ok-title="$t('tsSave')" :cancel-title="$t('tsCancel')" scrollable @ok='saveSeconds'>
			<b-form-group :id='"star" + i' :key='i' :label="$t('tsStar', {n: i})" label-for='time' label-cols-sm='4' v-for='i in 8'>
				<b-form-select v-model.number='starModalSeconds[i - 1]' id='time' :options='starSecondOptions'>
				</b-form-select>
			</b-form-group>
		</b-modal>
	</div>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import {DayInfo, DayType, AmbiguousWeather, ShowerType, StarInfo, dayUsesTypes, getPossiblePatternsForDay, getPatternName, rainbowPatternsByTime, DayForecast} from '../model'
import {isSpecialDay, SpecialDay, Hemisphere, SpWeatherLevel, getSpWeatherLevel, Weather, Pattern} from '../../pkg'
import { TranslateResult } from 'vue-i18n'

@Component
export default class DayEditor extends Vue {
	@Prop(Object) readonly day!: DayInfo
	@Prop(Number) readonly hemisphere!: Hemisphere

	get hourOptions() {
		const options = []
		for (let i = 5; i < 24; i++)
			options.push({value: i, text: this.$d(new Date(0, 0, 0, i), 'timeHOnly')})
		for (let i = 0; i < 5; i++)
			options.push({value: i, text: this.$d(new Date(0, 0, 0, i), 'timeHOnly')})
		return options
	}
	get weatherTypeOptions() {
		return [
			{value: Weather.Clear, text: this.$t('lstPatternChoices0')},
			{value: Weather.Sunny, text: this.$t('lstPatternChoices1')},
			{value: Weather.Cloudy, text: this.$t('lstPatternChoices2')},
			{value: Weather.RainClouds, text: this.$t('lstPatternChoices3')},
			{value: Weather.Rain, text: this.$t('lstPatternChoices4')},
			{value: Weather.HeavyRain, text: this.$t('lstPatternChoices5')},
			{value: AmbiguousWeather.ClearOrSunny, text: this.$t('lstPatternChoices6')},
			{value: AmbiguousWeather.SunnyOrCloudy, text: this.$t('lstPatternChoices7')},
			{value: AmbiguousWeather.CloudyOrRainClouds, text: this.$t('lstPatternChoices8')},
			{value: AmbiguousWeather.NoRain, text: this.$t('lstPatternChoices9')},
			{value: AmbiguousWeather.RainOrHeavyRain, text: this.$t('lstPatternChoices10')},
		]
	}
	get starHourOptions() {
		const options = []
		for (let i = 19; i < 24; i++)
			options.push({value: i, text: this.$d(new Date(0, 0, 0, i), 'timeHOnly')})
		for (let i = 0; i < 4; i++)
			options.push({value: i, text: this.$d(new Date(0, 0, 0, i), 'timeHOnly')})
		return options
	}
	get minuteOptions() {
		const options = []
		for (let i = 0; i < 60; i++)
			options.push({value: i, text: ('0' + i).slice(-2)})
		return options
	}
	get starSecondOptions() {
		const options = [{value: 99, text: this.$t('dunno')}]
		for (let i = 0; i < 60; i++)
			options.push({value: i, text: ('0' + i).slice(-2)})
		return options
	}
	get rainbowTimeOptions() {
		const options = []
		for (const hour of Object.keys(rainbowPatternsByTime))
			options.push({value: hour, text: this.$d(new Date(0, 0, 0, parseInt(hour, 10)), 'timeH')})
		return options
	}

	_computeAuroraGaps(pairs: Date[][]): string {
		const bits = []
		for (const pair of pairs) {
			bits.push(this.$d(pair[0], 'timeH') + '-' + this.$d(pair[1], 'timeH'))
		}
		return bits.join(', ')
	}
	get labelAuroraFine01() {
		return this._computeAuroraGaps([
			[new Date(0,0,0,22), new Date(0,0,0,23)],
			[new Date(0,0,0,0), new Date(0,0,0,2)],
			[new Date(0,0,0,3), new Date(0,0,0,4)],
		])
	}
	get labelAuroraFine03() {
		return this._computeAuroraGaps([
			[new Date(0,0,0,19), new Date(0,0,0,20)],
			[new Date(0,0,0,21), new Date(0,0,0,23)],
			[new Date(0,0,0,0), new Date(0,0,0,1)],
			[new Date(0,0,0,2), new Date(0,0,0,4)],
		])
	}
	get labelAuroraFine05() {
		return this._computeAuroraGaps([
			[new Date(0,0,0,22), new Date(0,0,0,23)],
			[new Date(0,0,0,3), new Date(0,0,0,4)],
		])
	}

	get specialDayWarning(): TranslateResult {
		const spDay = this.specialDay
		if (spDay != SpecialDay.None) {
			const hemi = this.$t((this.hemisphere == Hemisphere.Northern) ? 'tnorthern' : 'tsouthern')
			const day = this.$t('lstSpecialDaysA' + (spDay - 1))
			let warn
			switch (spDay) {
				case SpecialDay.FishCon:
				case SpecialDay.InsectCon:
					warn = this.$t('tSpDayWarnRS')
					break
				case SpecialDay.Easter:
					warn = this.$t('tSpDayWarnEv')
					break
				default:
					warn = this.$t('tSpDayWarn')
			}

			return '⚠️ ' + this.$t('tSpecialDay', {hemi, day, warn})
		}
		return ''
	}



	starModalIndex: number = 0
	starModalSeconds: number[] = []

	get specialDay(): SpecialDay {
		return isSpecialDay(this.hemisphere, this.day.y, this.day.m, this.day.d)
	}
	get spWeatherLevel(): SpWeatherLevel {
		return getSpWeatherLevel(this.hemisphere, this.day.m, this.day.d)
	}

	get hasAuroraOption(): boolean {
		return this.day.dayType == DayType.Aurora || this.spWeatherLevel == SpWeatherLevel.Aurora
	}
	get hasRainbowOption(): boolean {
		return this.day.dayType == DayType.Rainbow || this.spWeatherLevel == SpWeatherLevel.Rainbow
	}
	get isShowerMode(): boolean { return this.day.dayType == DayType.Shower }
	get isRainbowMode(): boolean { return this.day.dayType == DayType.Rainbow }
	get isAuroraMode(): boolean { return this.day.dayType == DayType.Aurora }
	get isTypesMode(): boolean { return dayUsesTypes(this.day) }

	get possiblePatterns(): Pattern[] {
		return getPossiblePatternsForDay(this.hemisphere, this.day)
	}
	get possiblePatternNames(): string[] {
		return this.possiblePatterns.map(getPatternName)
	}
	demoPattern(patName: keyof typeof Pattern) {
		const dayFC = new DayForecast(
			this.hemisphere, null,
			this.day.y, this.day.m, this.day.d,
			Pattern[patName]
		)
		this.$emit('show-day', dayFC)
	}

	// Shower UI
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
		return (seconds.length == 0) ? this.$t('tEditSeconds') : seconds.join(', ')
	}
	get starSecondsTitle(): TranslateResult {
		const star = this.day.stars[this.starModalIndex]
		if (star === undefined) {
			return ''
		} else {
			const date = new Date(0, 0, 0, star.hour, star.minute)
			return this.$t('tsTitle', {time: this.$d(date, 'timeHM')})
		}
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

	addGap() {
		let hour = 19, minute = 0
		if (this.day.gaps.length > 0) {
			const last = this.day.gaps[this.day.gaps.length - 1]
			hour = last.endHour
			minute = last.endMinute + 1
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
		this.day.gaps.push({startHour: hour, startMinute: minute, endHour: hour, endMinute: minute})
	}
	removeGap(index: number) {
		this.day.gaps.splice(index, 1)
	}
	makeGapsFromTimes() {
		this.day.gaps.splice(0, this.day.gaps.length)

		const starsSorted = this.day.stars.slice()
		starsSorted.sort((a: StarInfo, b: StarInfo) => {
			let aLinear = (a.hour < 5) ? (a.hour + 5) : (a.hour - 19)
			let bLinear = (b.hour < 5) ? (b.hour + 5) : (b.hour - 19)
			if (aLinear < bLinear) {
				return -1
			} else if (aLinear > bLinear) {
				return 1
			} else {
				if (a.minute < b.minute) {
					return -1
				} else if (a.minute > b.minute) {
					return 1
				} else {
					return 0
				}
			}
		})

		for (let i = 1; i < starsSorted.length; i++) {
			const a = starsSorted[i - 1]
			const b = starsSorted[i]
			let startHour = a.hour, startMinute = a.minute + 1
			if (startMinute == 60) {
				startHour += 1
				startMinute = 0
				if (startHour == 24) startHour = 0
			}
			let endHour = b.hour, endMinute = b.minute - 1
			if (endMinute == 0) {
				endHour -= 1
				endMinute = 59
				if (endHour == -1) endHour = 23
			}
			if (b.hour != startHour || b.minute != startMinute) {
				this.day.gaps.push({startHour, startMinute, endHour, endMinute})
			}
		}
	}

	// Weather types UI
	get typesListFull(): boolean {
		return this.possiblePatterns.length == 1
	}

	addType() {
		let hour = 5
		if (this.day.types.length > 0) {
			hour = this.day.types[this.day.types.length - 1].time
			if (hour == 23)
				hour = 0
			else
				hour += 1
		}
		this.day.types.push({time: hour, type: Weather.Clear})
	}
	removeType(index: number) {
		this.day.types.splice(index, 1)
	}
}
</script>
