<template>
	<div>
		<h2 class='mt-3'>{{ $t('hTWelcome') }}</h2>
		<div v-html="$t('hWelcome')"></div>
		<h3 class='mt-3'>{{ $t('hTLog') }}</h3>
		<ul>
			<li v-for='(entry, index) in changelogEntries' :key='index'>
				<b>{{ $d(entry.date, 'short') }}</b>: {{ entry.text }}
			</li>
		</ul>
		<h3 class='mt-3'>{{ $t('hTFAQ') }}</h3>
		<template v-for='key in faqQuestions'>
			<h5 class='mt-3' :key="'q' + key">{{ $t('hQ' + key) }}</h5>
			<div v-html="timeify($t('hA' + key))" :key="'a' + key"></div>
		</template>
	</div>
</template>

<script lang='ts'>
import { Vue, Component } from 'vue-property-decorator'
import { makeTime } from '../utils'

@Component
export default class WelcomePage extends Vue {
	get changelogEntries() {
		const entries = []
		for (const key of Object.keys(this.$i18n.messages.en)) {
			if (key.startsWith('hLog')) {
				const y = parseInt(key.slice(4, 8), 10)
				const m = parseInt(key.slice(8, 10), 10)
				const d = parseInt(key.slice(10, 12), 10)
				entries.push({date: new Date(y, m - 1, d), text: this.$t(key)})
			}
		}
		return entries
	}

	timeify(str: string): string {
		// accessing time12 here forces this component to be
		// updated whenever time12 changes...
		const awfulHack = (this.$root as any).time12

		return str.replace(/[012]\d:\d\d:\d\d/g, time => {
			const [hS, mS, sS] = time.split(':')
			const h = parseInt(hS, 10), m = parseInt(mS, 10), s = parseInt(sS, 10)
			return this.$d(makeTime(h, m, s), 'timeHMS')
		}).replace(/[012]\d:\d\d/g, time => {
			const [hS, mS] = time.split(':')
			const h = parseInt(hS, 10), m = parseInt(mS, 10)
			return this.$d(makeTime(h, m), 'timeHM')
		}).replace(/[012]\dH/g, time => {
			const h = parseInt(time.slice(0, 2), 10)
			return this.$d(makeTime(h, 0), 'timeH')
		})
	}

	get faqQuestions() {
		return ['Seed', 'Celeste', 'Wind', 'StarVis', 'Star130', 'MoreData', 'NoSeeds', 'BadSeed', 'Add', 'Donate', 'Source']
	}
}
</script>

