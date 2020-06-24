<template>
<div>
	<b-navbar variant='light' toggleable='md'>
		<b-container>
			<b-navbar-brand tag='span'>MeteoNook Alpha</b-navbar-brand>
			<b-dropdown id='dropdown-settings' :text="$t('mSettings')" ref='dropdown'>
				<b-dropdown-header>{{ $t('mTimeFormat') }}</b-dropdown-header>
				<b-dropdown-item-button>{{ $t('mTime12') }}</b-dropdown-item-button>
				<b-dropdown-item-button>{{ $t('mTime24') }}</b-dropdown-item-button>
				<b-dropdown-divider></b-dropdown-divider>
				<b-dropdown-header>{{ $t('mLanguage') }}</b-dropdown-header>
				<b-dropdown-item-button v-for='lang in languages' :key='lang'>
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
				<yearly-forecast :forecast='forecast' @switch-to-monthly='switchToMonthly'></yearly-forecast>
			</b-tab>
			<b-tab :title="$t('mTab')" ref='monthlyTab'>
				<monthly-forecast :forecast='forecast' @show-day='showDayModal'></monthly-forecast>
			</b-tab>
		</b-tabs>
	</b-container>

	<day-modal id='dayModal' :forecast='forecast' :date='dayModalDate'></day-modal>
</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'
import WelcomePage from './WelcomePage.vue'
import SeedFinder from './SeedFinder.vue'
import YearlyForecast from './YearlyForecast.vue'
import MonthlyForecast from './MonthlyForecast.vue'
import DayModal from './DayModal.vue'
import { Forecast } from '../model'
import { BTab } from 'bootstrap-vue'

@Component({components: {WelcomePage, SeedFinder, YearlyForecast, MonthlyForecast, DayModal}})
export default class App extends Vue {
	forecast = new Forecast()
	dayModalDate = new Date()

	$refs!: {
		monthlyTab: BTab
	}

	switchToMonthly() {
		this.$refs.monthlyTab.activate()
	}

	showDayModal(year: number, month: number, day: number) {
		this.dayModalDate = new Date(year, month, day)
		this.$bvModal.show('dayModal')
	}

	get languages() {
		return this.$i18n.availableLocales
	}
	getLanguageName(key: string) {
		return this.$i18n.getLocaleMessage(key).lang
	}
}
</script>
