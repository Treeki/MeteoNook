<template>
	<b-form class='border rounded p-3'>
		<h5>{{ $t('tForecastConfig') }}</h5>
		<b-form-group
			:label="$t('tIslandName')"
			:description="$t('tIslandNameDesc')"
			label-cols-sm='4' label-for='name'>
			<b-form-input type='text' id='name' v-model.trim='island.name'></b-form-input>
		</b-form-group>
		<b-form-group
			:label="$t('tHemisphere')"
			label-class='pt-0'
			label-cols-sm='4'>
			<b-form-radio-group v-model.number='island.hemisphere'>
				<b-form-radio value='0'>{{ $t('tNorthern') }}</b-form-radio>
				<b-form-radio value='1'>{{ $t('tSouthern') }}</b-form-radio>
			</b-form-radio-group>
		</b-form-group>
		<b-form-group
			:label="$t('tSeed')"
			label-cols-sm='4' label-for='seed'>
			<b-form-input type='number' id='seed' min='0' max='2147483647' v-model.number='island.seed'></b-form-input>
		</b-form-group>
		<b-form-group
			:label="$t('tClockMode')"
			:description="$t('tClockModeDesc')"
			label-class='pt-0'
			label-cols-sm='4'>
			<b-form-radio-group stacked v-model.number='clockMode'>
				<b-form-radio value='0'>{{ $t('tRealTime') }}</b-form-radio>
				<b-form-radio value='1'>{{ $t('tTimeTravel') }}</b-form-radio>
			</b-form-radio-group>
		</b-form-group>
		<b-form-group
			:label="$t('tClockOffset')"
			:description="$t('tClockOffsetDesc')"
			label-cols-sm='4'
			v-show='clockMode == 1'>
			<b-input-group>
				<b-form-input type='number' id='offset' min='-31557600' max='31557600' v-model.number='island.offsetMinutes'></b-form-input>
				<b-input-group-append>
					<b-input-group-text>{{ $t('tMinutesSuffix') }}</b-input-group-text>
				</b-input-group-append>
			</b-input-group>
			<b-form-datepicker class='mt-2' label-help='' value-as-date :value='gameDate' @input='gameDateSet'></b-form-datepicker>
			<b-form-timepicker class='mt-2' :value='gameTime' @input='gameTimeSet'></b-form-timepicker>
		</b-form-group>
		<b-button variant='outline-primary' @click='preview'>{{ $t('tPreview') }}</b-button>
		<b-button variant='primary' @click='save'>{{ $t('tSave') }}</b-button>
		<b-button variant='danger' @click='cancel' v-show='allowCancel'>{{ $t('tCancel') }}</b-button>
	</b-form>
</template>

<script lang="ts">
import { Vue, Component, Prop, Watch } from 'vue-property-decorator'
import { Forecast, IslandInfo } from '../model'

@Component
export default class ForecastConfig extends Vue {
	island = new IslandInfo()
	clockMode: number = 0
	gameDate: Date = new Date()
	gameTime: string = '00:00:00'
	minuteChangeCallbackId: number|null = null
	@Prop(Boolean) allowCancel!: boolean

	created() {
		this._scheduleMinuteChangeCallback()
	}
	beforeDestroy() {
		if (this.minuteChangeCallbackId !== null)
			window.clearTimeout(this.minuteChangeCallbackId)
	}

	setBaseData(island: IslandInfo) {
		this.island = new IslandInfo(island)
		this.clockMode = (island.offsetMinutes == 0) ? 0 : 1
	}
	preview() { this.$emit('preview', new IslandInfo(this.island)) }
	save() { this.$emit('save', new IslandInfo(this.island)) }
	cancel() { this.$emit('cancel') }


	_scheduleMinuteChangeCallback() {
		const d = new Date()
		const now = d.getTime()
		d.setTime(now + 60_000)
		d.setSeconds(0)
		d.setMilliseconds(0)
		const delta = (d.getTime() - now) + 50
		this.minuteChangeCallbackId = window.setTimeout(() => {
			this._setFromIslandOffset()
			this._scheduleMinuteChangeCallback()
		}, delta)
	}

	gameDateSet(date: Date) {
		this.gameDate = date
		this._setFromGameInputs()
	}
	gameTimeSet(time: string) {
		this.gameTime = time
		this._setFromGameInputs()
	}

	_setFromGameInputs() {
		const here = new Date()
		here.setSeconds(0)
		here.setMilliseconds(0)

		const game = new Date()
		game.setTime(this.gameDate.getTime())
		const [h, m, s] = this.gameTime.split(':')
		game.setHours(parseInt(h, 10))
		game.setMinutes(parseInt(m, 10))

		this.island.offsetMinutes = Math.floor((game.getTime() - here.getTime()) / 60_000)
	}
	@Watch('island.offsetMinutes', {immediate: true})
	_setFromIslandOffset() {
		const here = new Date()
		here.setSeconds(0)
		here.setMilliseconds(0)
		const game = new Date()
		game.setTime(here.getTime() + this.island.offsetMinutes * 60_000)

		this.gameDate = game
		this.gameTime = ('0' + game.getHours()).slice(-2) + ':' + ('0' + game.getMinutes()).slice(-2) + ':00'
	}
}
</script>
