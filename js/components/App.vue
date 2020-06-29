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
			<b-tab :title="$t('hTab')" active>
				<welcome-page></welcome-page>
			</b-tab>
			<b-tab :title="$t('sTab')">
				<seed-finder></seed-finder>
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
import YearlyView from './YearlyView.vue'
import MonthlyView from './MonthlyView.vue'
import DayModal from './DayModal.vue'
import CreditsModal from './CreditsModal.vue'
import { Forecast, DayForecast, Hemisphere } from '../model'
import { BTab } from 'bootstrap-vue'
import { LocaleMessage, DateTimeFormat } from 'vue-i18n'
import { writeStorage } from '../utils'

@Component({components: {WelcomePage, SeedFinder, YearlyView, MonthlyView, DayModal, CreditsModal}})
export default class App extends Vue {
	forecast = new Forecast()
	dayModalData = new DayForecast(Hemisphere.Northern, 0, 1970, 1, 1)

	$refs!: {
		monthlyTab: BTab
	}

	switchToMonthly() {
		this.$refs.monthlyTab.activate()
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
}
</script>
