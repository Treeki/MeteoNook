<style lang="scss">
@import '~bootstrap/scss/functions';
@import '~bootstrap/scss/variables';
@import '~bootstrap/scss/mixins';

.card-columns {
	column-count: 1;
	@include media-breakpoint-only(md) { column-count: 2; }
	@include media-breakpoint-only(lg) { column-count: 3; }
	@include media-breakpoint-up(xl) { column-count: 4; }
}
</style>

<template>
	<div>
		<b-alert :show='storedIslands.length == 0'>
			{{ $t('oExample') }}
		</b-alert>
		<forecast-config
			ref='config'
			class='mb-3'
			v-show='showEditor'
			@preview='previewIsland'
			@save='saveIsland'
			@cancel='cancelEditing'
			:allow-cancel='canCancelEdit'
		></forecast-config>
		<h5 class='mb-3' v-show='!showEditor'>
			{{ $t('oTitle', {island: forecast.islandName}) }}
			<b-dropdown size='sm' id='dropdown-forecast' :text="$t('oChange')" ref='dropdown' right class='float-right'>
				<b-dropdown-header>{{ forecast.islandName }}</b-dropdown-header>
				<b-dropdown-item-button @click='editIsland'>{{ $t('oEditSettings') }}</b-dropdown-item-button>
				<b-dropdown-item-button @click='doShareLink'>{{ $t('oShareLink') }}</b-dropdown-item-button>
				<b-dropdown-item-button @click='removeIsland'>{{ $t('oRemoveIsland') }}</b-dropdown-item-button>
				<b-dropdown-divider></b-dropdown-divider>
				<b-dropdown-header>{{ $t('oSavedIslands') }}</b-dropdown-header>
				<b-dropdown-item-button
					v-for='(island, index) in storedIslands'
					:key='index'
					:active='island == activeIsland'
					@click='selectIsland(index)'>
					{{ island.name }}
				</b-dropdown-item-button>
				<b-dropdown-divider></b-dropdown-divider>
				<b-dropdown-item-button @click='addIsland'>{{ $t('oAddIsland') }}</b-dropdown-item-button>
			</b-dropdown>

			<b-modal :title="$t('lsTitle')" id='linkShare' ok-only :ok-title="$t('lsDone')" centered>
				<p>{{ $t('lsBlurb') }}</p>
				<b-form-input type='text' readonly :value='shareURL'></b-form-input>
			</b-modal>
		</h5>
		<b-alert :show='guidanceModeStr.length > 0'>
			{{ $t(guidanceModeStr) }}
		</b-alert>
		<b-card-group columns>
			<overview-day-card :title="$t('oToday')" :forecast='forecast' :day='today' @show-day='showDay'></overview-day-card>
			<overview-day-card :title="$t('oTomorrow')" :forecast='forecast' :day='tomorrow' @show-day='showDay'></overview-day-card>
			<b-card :title="$t('oUpcoming')">
				<b-card-text class='mb-0' v-for='(event, index) in upcomingEvents' :key='index'>
					<b>{{ $d(event.date, 'weekDayMonth') }}</b>
					<b-card-text v-for='(text, index2) in event.texts' :key='index2'>
						{{ text }}
					</b-card-text>
				</b-card-text>
			</b-card>
			<b-card :title="$t('oConstellation')">
				<b-card-text>
					<b>{{ $t('oZodiacNow') }}</b>
					{{ $t('lstConstellation' + zodiac.current) }}
				</b-card-text>
				<b-card-text>
					<b>{{ $t('oZodiacNext') }}</b>
					{{ $t('lstConstellation' + zodiac.next) }}
					<i>{{ $t('oZodiacFrom', {date: $d(zodiac.nextDate, 'weekDayMonth')}) }}</i>
				</b-card-text>
			</b-card>
		</b-card-group>
	</div>
</template>

<script lang="ts">
import { Vue, Component, Prop } from 'vue-property-decorator'
import { Forecast, DayForecast, IslandInfo } from '../model'
import ForecastConfig from './ForecastConfig.vue'
import OverviewDayCard from './OverviewDayCard.vue'
import { TranslateResult } from 'vue-i18n'
import { SnowLevel, SpWeatherLevel, FogLevel, getConstellation, Hemisphere } from '../../pkg'

interface EventInfo {
	date: Date,
	texts: TranslateResult[]
}

enum EditMode {
	Closed,
	AddNew,
	EditExisting
}
enum GuidanceMode {
	None,
	SingleSeed,
	MultiSeed
}

@Component({components: {ForecastConfig, OverviewDayCard}})
export default class ForecastOverview extends Vue {
	@Prop(Forecast) readonly forecast!: Forecast
	@Prop(IslandInfo) readonly activeIsland!: IslandInfo
	@Prop(Array) readonly storedIslands!: IslandInfo[]
	editMode = EditMode.Closed
	guidanceMode = GuidanceMode.None

	$refs!: {
		config: ForecastConfig
	}

	get zodiac() {
		const current = this.today.constellation
		for (const day of this.upcomingDays) {
			if (day.constellation != current) {
				const next = day.constellation
				const nextDate = day.date
				return {current, next, nextDate}
			}
		}
		// weird error case
		// should never happen as we should always have >30 days in upcomingDays
		return {current, next: current, nextDate: new Date()}
	}

	get upcomingDays(): DayForecast[] {
		const days = []
		const date = this.forecast.todayDate
		for (let i = 0; i < 35; i++) {
			days.push(new DayForecast(
				this.forecast.hemisphere, this.forecast.seed,
				date.getFullYear(), date.getMonth() + 1, date.getDate()
			))
			date.setTime(date.getTime() + 86400_000)
		}
		return days
	}

	get upcomingEvents(): EventInfo[] {
		// track changes
		const today = this.upcomingDays[0]
		let prevSnowLevel = today.snowLevel
		let prevFogLevel = today.fogLevel

		const events = []
		for (const day of this.upcomingDays) {
			const date = day.date
			const texts = []
			if (day.rainbowCount > 0) {
				const rainbowDate = new Date(day.date)
				rainbowDate.setHours(day.rainbowHour)
				const prefix = (day.rainbowCount == 2) ? '🌈🌈 ' : '🌈 '
				texts.push(prefix + this.$t('oRainbowAt', {time: this.$d(rainbowDate, 'timeHM')}))
			}
			if (day.aurora)
				texts.push('🌌 ' + this.$t('tAurora' + this.forecast.hemiSuffix))
			if (day.heavyShower)
				texts.push('🌠 ' + this.$t('oHeavyShower'))

			if (prevSnowLevel != day.snowLevel) {
				if (day.snowLevel == SnowLevel.None)
					texts.push('❄️ ' + this.$t('oSnowEnd'))
				else if (day.snowLevel == SnowLevel.Low)
					texts.push('❄️ ' + this.$t('oSnowStart'))
			}
			if (prevFogLevel != day.fogLevel) {
				if (day.fogLevel == FogLevel.None)
					texts.push('️🌫 ' + this.$t('oFogEnd'))
				else if (day.fogLevel == FogLevel.HeavyAndWater)
					texts.push('️🌫 ' + this.$t('oFogStart'))
			}
			prevSnowLevel = day.snowLevel
			prevFogLevel = day.fogLevel

			if (texts.length > 0)
				events.push({date, texts})
			if (events.length > 5)
				break
		}
		return events
	}

	get today(): DayForecast { return this.upcomingDays[0] }
	get tomorrow(): DayForecast { return this.upcomingDays[1] }



	get showEditor(): boolean {
		return (this.editMode != EditMode.Closed) || (this.storedIslands.length == 0)
	}
	get canCancelEdit(): boolean {
		return (this.storedIslands.length > 0)
	}
	previewIsland(island: IslandInfo) {
		this.$emit('set-preview-island', island)
	}
	saveIsland(island: IslandInfo) {
		if (this.editMode == EditMode.EditExisting)
			this.$emit('update-island', island)
		else
			this.$emit('add-island', island)
		this.editMode = EditMode.Closed
		this.guidanceMode = GuidanceMode.None
	}

	// called externally from App.vue
	setPreviewSeedFromFinder(seed: number, hemisphere: Hemisphere, multiFlag: boolean) {
		this.editMode = EditMode.AddNew
		const island = new IslandInfo()
		island.seed = seed
		island.hemisphere = hemisphere
		this.$refs.config.setBaseData(island)
		this.previewIsland(island)
		this.guidanceMode = multiFlag ? GuidanceMode.MultiSeed : GuidanceMode.SingleSeed
	}

	addIsland() {
		this.editMode = EditMode.AddNew
		this.$refs.config.setBaseData(new IslandInfo())
	}
	editIsland() {
		this.editMode = EditMode.EditExisting
		this.$refs.config.setBaseData(this.activeIsland!)
	}
	cancelEditing() {
		this.editMode = EditMode.Closed
		this.$emit('cancel-preview')
	}
	removeIsland() {
		this.$emit('remove-island')
	}

	selectIsland(index: number) {
		this.$emit('select-island', index)
	}


	get guidanceModeStr(): string {
		switch (this.guidanceMode) {
			case GuidanceMode.None: return ''
			case GuidanceMode.SingleSeed: return 'oGuidanceSingle'
			case GuidanceMode.MultiSeed: return 'oGuidanceMulti'
		}
	}


	showDay(day: DayForecast) { this.$emit('show-day', day) }


	doShareLink() {
		this.$bvModal.show('linkShare')
	}
	get shareURL(): string {
		const url = new URL(document.location.href)
		url.search = this.activeIsland.queryString
		return url.href
	}
}

</script>