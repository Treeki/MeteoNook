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
			<div v-html="$t('hA' + key)" :key="'a' + key"></div>
		</template>
	</div>
</template>

<script lang='ts'>
import { Vue, Component } from 'vue-property-decorator'

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

	get faqQuestions() {
		return ['Seed', 'Celeste', 'Wind', 'MoreData', 'NoSeeds', 'BadSeed', 'Add', 'Donate', 'Source']
	}
}
</script>

