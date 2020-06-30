<template>
<div>
	<b-navbar variant='light' toggleable='md'>
		<b-container>
			<b-navbar-brand tag='span'>MeteoNook Alpha</b-navbar-brand>
			<b-dropdown id='dropdown-settings' :text="$t('mSettings')" ref='dropdown' right>
				<b-dropdown-header>{{ $t('mTimeFormat') }}</b-dropdown-header>
				<b-dropdown-item-button :active='time12' @click='setTime12'>{{ $t('mTime12') }}</b-dropdown-item-button>
				<b-dropdown-item-button :active='!time12' @click='setTime24'>{{ $t('mTime24') }}</b-dropdown-item-button>
				<b-dropdown-divider></b-dropdown-divider>
				<b-dropdown-header>{{ $t('mLanguage') }}</b-dropdown-header>
				<b-dropdown-item-button
					v-for='lang in languages'
					:active='lang == currentLanguage'
					:key='lang'
					@click='setLanguage(lang)'
				>
					{{ getLanguageName(lang) }}
				</b-dropdown-item-button>
			</b-dropdown>
		</b-container>
	</b-navbar>

	<b-container fluid='md'>
		<b-tabs class='mt-2 mx-0' content-class='mt-3'>
			<b-tab :title="$t('hTab')" :active='!hasIslandAtLoad'>
				<welcome-page></welcome-page>
			</b-tab>
			<b-tab :title="$t('sTab')">
				<seed-finder
					@preview-seed='setPreviewSeedFromFinder'
					@show-day='showDayModal'
					>
				</seed-finder>
			</b-tab>
			<b-tab :title="$t('oTab')" :active='hasIslandAtLoad' ref='overviewTab'>
				<forecast-overview
					ref='overview'
					:forecast='forecast'
					:stored-islands='storedIslands'
					:active-island='currentIsland'
					@add-island='addIslandAndMakeCurrent'
					@update-island='updateCurrentIsland'
					@set-preview-island='setPreviewIsland'
					@select-island='selectIsland'
					@remove-island='removeIsland'
					@cancel-preview='cancelPreviewIsland'
					@show-day='showDayModal'
					>
				</forecast-overview>
			</b-tab>
			<b-tab :title="$t('yTab')">
				<yearly-view :forecast='forecast' @switch-to-monthly='switchToMonthly'></yearly-view>
			</b-tab>
			<b-tab :title="$t('mTab')" ref='monthlyTab'>
				<monthly-view :forecast='forecast' :month='forecast.currentMonth' @show-day='showDayModal'></monthly-view>
			</b-tab>
		</b-tabs>

		<footer class='my-5 pt-2 border-top text-muted'>
			<p class='mb-2'>
				<i18n tag='span' path='footerCopy'>
					<template v-slot:me>Ash Wolf (<a href='https://twitter.com/_Ninji'>@_Ninji</a>)</template>
					<template v-slot:year>2020</template>
				</i18n>
				<b-link href='#' v-b-modal.credits-modal>{{ $t('credTitle') }}</b-link>
			</p>
			<p class='mb-2'>
				{{ $t('footerDonate') }}
				<a href='https://paypal.me/trashcurl'>PayPal.me</a>
				|
				<a href='https://ko-fi.com/ninji_'>Ko-fi</a>
				|
				<a href='https://monzo.me/ninji'>Monzo</a> (UK)
			</p>
			<p>
				{{ $t('footerVersion') }}
				<a :href='gitCommitUrl'>{{ gitCommitShort }}</a> ({{ gitCommitStamp }})
			</p>
		</footer>
	</b-container>

	<day-modal id='dayModal' :forecast='forecast' :day='dayModalData'></day-modal>
	<credits-modal id='credits-modal'></credits-modal>
</div>
</template>

<script lang='ts'>
declare var METEONOOK_GIT_COMMIT_SHORT: string
declare var METEONOOK_GIT_COMMIT_URL: string
declare var METEONOOK_GIT_COMMIT_STAMP: string

import Vue from 'vue'
import Component from 'vue-class-component'
import WelcomePage from './WelcomePage.vue'
import SeedFinder from './SeedFinder.vue'
import ForecastOverview from './ForecastOverview.vue'
import YearlyView from './YearlyView.vue'
import MonthlyView from './MonthlyView.vue'
import DayModal from './DayModal.vue'
import CreditsModal from './CreditsModal.vue'
import { Forecast, DayForecast, Hemisphere, IslandInfo } from '../model'
import { BTab } from 'bootstrap-vue'
import { LocaleMessage, DateTimeFormat } from 'vue-i18n'
import { writeStorage, readStorageObject, writeStorageObject } from '../utils'
import { Watch } from 'vue-property-decorator'

interface IslandInfoHolder {
	version: number,
	islands: IslandInfo[],
	currentIndex: number|null
}

function loadIslands(): IslandInfoHolder {
	let info = readStorageObject<IslandInfoHolder>('meteonook_islands', (iih) => iih.version == 2)
	if (info === null) {
		info = {version: 2, islands: [], currentIndex: null}
	}

	const islands = info.islands
	let immediateSave = false

	// check the URL for an imported island
	if (document !== undefined && document.location.search.startsWith('?v1&')) {
		const bits = document.location.search.split('&')
		if (bits.length === 4) {
			const island = new IslandInfo()
			island.name = decodeURIComponent(bits[1])
			island.seed = parseInt(decodeURIComponent(bits[2]), 10)
			island.hemisphere = (decodeURIComponent(bits[3]).toUpperCase() == 'S') ? Hemisphere.Southern : Hemisphere.Northern

			// is there one like it in the array?
			const found = islands.findIndex(e => (e.name == island.name && e.seed == island.seed && e.hemisphere == island.hemisphere))
			if (found > -1) {
				// this already exists
				if (info.currentIndex !== found) {
					info.currentIndex = found
					immediateSave = true
				}
			} else {
				// new island, add it
				islands.push(island)
				info.currentIndex = islands.length - 1
				immediateSave = true
			}
		}
	}

	if (info.currentIndex === null && islands.length > 0)
		info.currentIndex = 1

	if (immediateSave)
		writeStorageObject('meteonook_islands', info)

	return info
}

function saveIslands(islands: IslandInfo[], currentIndex: number|null) {
	const info: IslandInfoHolder = {
		version: 2, islands, currentIndex
	}
	writeStorageObject('meteonook_islands', info)
}


@Component({components: {WelcomePage, SeedFinder, YearlyView, MonthlyView, ForecastOverview, DayModal, CreditsModal}})
export default class App extends Vue {
	currentIslandIndex!: number|null
	previewIsland!: IslandInfo
	storedIslands!: IslandInfo[]
	forecast!: Forecast
	previewOverride: boolean = false
	dayModalData = new DayForecast(Hemisphere.Northern, 0, 1970, 1, 1)

	data() {
		// these need to be specified using data hook because
		// of inter-dependent object fuckery
		const data = loadIslands()
		console.log('INITIAL DATA', data)
		const previewIsland = new IslandInfo()
		const startIsland = (data.currentIndex !== null) ? data.islands[data.currentIndex] : previewIsland

		return {
			currentIslandIndex: data.currentIndex,
			storedIslands: data.islands,
			previewIsland,
			forecast: new Forecast(startIsland)
		}
	}

	$refs!: {
		monthlyTab: BTab,
		overviewTab: BTab,
		overview: ForecastOverview
	}

	switchToMonthly() {
		this.$refs.monthlyTab.activate()
	}

	get hasIslandAtLoad(): boolean {
		return (this.currentIslandIndex !== null)
	}

	showDayModal(day: DayForecast) {
		this.dayModalData = day
		this.$bvModal.show('dayModal')
	}

	get languages(): string[] {
		return this.$i18n.availableLocales.filter(k => this.$i18n.getLocaleMessage(k).lang !== undefined)
	}
	get currentLanguage(): string {
		return this.$i18n.locale
	}
	getLanguageName(key: string): LocaleMessage {
		return this.$i18n.getLocaleMessage(key).lang
	}
	setLanguage(key: string) {
		this.$root.$i18n.locale = key
		writeStorage('meteonook_language', key)
	}

	get currentIsland(): IslandInfo {
		if (this.currentIslandIndex === null)
			return this.previewIsland
		else
			return this.storedIslands[this.currentIslandIndex]
	}
	get previewOrCurrentIsland(): IslandInfo {
		if (this.previewOverride)
			return this.previewIsland
		else
			return this.currentIsland
	}
	selectIsland(index: number|null, forceSave?: boolean) {
		if (index !== this.currentIslandIndex || forceSave) {
			this.currentIslandIndex = index
			saveIslands(this.storedIslands, this.currentIslandIndex)
		}
		this.forecast.island = this.previewOrCurrentIsland
		this.forecast.regenerateForecasts()
	}
	setPreviewIsland(island: IslandInfo) {
		this.previewIsland = island
		this.previewOverride = true
		this.selectIsland(this.currentIslandIndex)
	}
	cancelPreviewIsland() {
		this.previewOverride = false
		this.selectIsland(this.currentIslandIndex)
	}
	updateCurrentIsland(island: IslandInfo) {
		if (this.currentIslandIndex !== null) {
			Vue.set(this.storedIslands, this.currentIslandIndex, island)
			this.previewOverride = false
			this.selectIsland(this.currentIslandIndex, true)
		}
	}
	addIslandAndMakeCurrent(island: IslandInfo) {
		this.storedIslands.push(island)
		this.previewOverride = false
		this.selectIsland(this.storedIslands.length - 1, true)
	}
	removeIsland() {
		if (this.currentIslandIndex !== null) {
			this.storedIslands.splice(this.currentIslandIndex, 1)
			this.selectIsland(this.storedIslands.length == 0 ? null : (this.storedIslands.length - 1), true)
		}
	}
	setPreviewSeedFromFinder(seed: number, hemisphere: Hemisphere, multiFlag: boolean) {
		this.$refs.overview.setPreviewSeedFromFinder(seed, hemisphere, multiFlag)
		this.$refs.overviewTab.activate()
	}

	get time12(): boolean { return (this.$root as any).time12 }
	setTime12() { this.setTimeFormat(true) }
	setTime24() { this.setTimeFormat(false) }
	setTimeFormat(time12: boolean) {
		(this.$root as any).setTime12(time12)
		writeStorage('meteonook_timeFormat', time12 ? '12' : '24')
	}

	get gitCommitUrl(): string { return METEONOOK_GIT_COMMIT_URL }
	get gitCommitShort(): string { return METEONOOK_GIT_COMMIT_SHORT }
	get gitCommitStamp(): string { return METEONOOK_GIT_COMMIT_STAMP }

	@Watch('forecast.islandName', {immediate: true})
	@Watch('$i18n.locale')
	syncIslandNameToTitle() {
		if (this.forecast.islandName !== 'Anyisle')
			document.title = this.$t('title', {island: this.forecast.islandName}) as string
	}
}
</script>
