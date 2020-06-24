<template>
	<div>
		<h2 class='mt-3'>{{ $t('hTWelcome') }}</h2>
		<div v-html="$t('hWelcome')"></div>
		<h2 class='mt-3'>{{ $t('hTLog') }}</h2>
		<ul>
			<li v-for='(entry, index) in changelogEntries' :key='index'>
				<b>{{ $d(entry.date) }}</b>: {{ entry.text }}
			</li>
		</ul>
	</div>
</template>

<script lang='ts'>
import Vue from 'vue'
import Component from 'vue-class-component'

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
}
</script>

