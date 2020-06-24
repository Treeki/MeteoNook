<template>
<div>
	<b-navbar variant='light' toggleable='md'>
		<b-container>
			<b-navbar-brand tag='span'>MeteoNook Alpha</b-navbar-brand>
			<b-dropdown id='dropdown-settings' :text="$t('mSettings')" ref='dropdown' right>
				<b-dropdown-header>{{ $t('mTimeFormat') }}</b-dropdown-header>
				<b-dropdown-item-button>{{ $t('mTime12') }}</b-dropdown-item-button>
				<b-dropdown-item-button>{{ $t('mTime24') }}</b-dropdown-item-button>
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
	</b-container>

	<day-modal id='dayModal' :forecast='forecast' :day='dayModalData'></day-modal>
</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import WelcomePage from './WelcomePage.vue'
import SeedFinder from './SeedFinder.vue'
import YearlyView from './YearlyView.vue'
import MonthlyView from './MonthlyView.vue'
import DayModal from './DayModal.vue'
import { Forecast, DayForecast, Hemisphere } from '../model'
import { BTab } from 'bootstrap-vue'
import { LocaleMessage } from 'vue-i18n'

@Component({components: {WelcomePage, SeedFinder, YearlyView, MonthlyView, DayModal}})
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
	}
}
</script>
