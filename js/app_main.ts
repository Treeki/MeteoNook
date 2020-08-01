import Vue from 'vue'
import { ButtonPlugin, CalendarPlugin, DropdownPlugin, FormCheckboxPlugin, FormRadioPlugin, LayoutPlugin, NavbarPlugin, TabsPlugin, FormGroupPlugin, ModalPlugin, FormSelectPlugin, InputGroupPlugin, LinkPlugin, SpinnerPlugin, ProgressPlugin, AlertPlugin, FormInputPlugin, FormPlugin, FormDatepickerPlugin, FormTimepickerPlugin, CardPlugin, FormTextareaPlugin } from 'bootstrap-vue'
import VueI18n, { DateTimeFormats, DateTimeFormat } from 'vue-i18n'
import ICS from 'vue-ics'

Vue.use(AlertPlugin)
Vue.use(ButtonPlugin)
Vue.use(CalendarPlugin)
Vue.use(CardPlugin)
Vue.use(DropdownPlugin)
Vue.use(FormPlugin)
Vue.use(FormCheckboxPlugin)
Vue.use(FormDatepickerPlugin)
Vue.use(FormInputPlugin)
Vue.use(FormGroupPlugin)
Vue.use(FormRadioPlugin)
Vue.use(FormSelectPlugin)
Vue.use(FormTextareaPlugin)
Vue.use(FormTimepickerPlugin)
Vue.use(InputGroupPlugin)
//Vue.use(IconsPlugin)
Vue.use(LayoutPlugin)
Vue.use(LinkPlugin)
Vue.use(ModalPlugin)
Vue.use(NavbarPlugin)
Vue.use(ProgressPlugin)
Vue.use(SpinnerPlugin)
Vue.use(TabsPlugin)
Vue.use(VueI18n)
Vue.use(ICS)

import '../custom.scss'

document.getElementById('loadingPlaceholder')?.remove()

const root = document.createElement('div')
root.id = 'app'
document.body.appendChild(root)

// TODO: do this on compile using some webpack voodoo, probably
function fixI18N(strs: {[key: string]: string}): {[key: string]: string} {
	for (const k of Object.keys(strs)) {
		if (k.startsWith('lst')) {
			const bits = strs[k].split('%%')
			for (let i = 0; i < bits.length; i++) {
				strs[k + i] = bits[i]
			}
		} else if (strs[k].indexOf('%%') > 0) {
			// generate HTML pieces
			const bits = []
			let ulFlag = false
			for (const bit of strs[k].split('%%')) {
				if (bit.startsWith('-')) {
					if (!ulFlag) {
						bits.push('<ul>')
						ulFlag = true
					}
					bits.push('<li>')
					bits.push(bit.slice(1))
					bits.push('</li>')
				} else {
					if (ulFlag) {
						bits.push('</ul>')
						ulFlag = false
					}
					bits.push('<p>')
					bits.push(bit)
					bits.push('</p>')
				}
			}
			if (ulFlag) {
				bits.push('</ul>')
			}
			strs[k] = bits.join('')
		}
	}
	return strs
}

const messages = {
	en: fixI18N(require('../i18n/en.json')),
	es: fixI18N(require('../i18n/es.json')),
	de: fixI18N(require('../i18n/de.json')),
	'de-CH': fixI18N(require('../i18n/de-CH.json')),
	fr: fixI18N(require('../i18n/fr.json')),
	it: fixI18N(require('../i18n/it.json')),
	ja: fixI18N(require('../i18n/ja.json')),
	ru: fixI18N(require('../i18n/ru.json')),
	'zh-Hans': fixI18N(require('../i18n/zh-Hans.json')),
	'zh-Hant': fixI18N(require('../i18n/zh-Hant.json')),
	'zh-TW': fixI18N(require('../i18n/zh-TW.json')),
	'en-GB': {'lang': 'English (UK)'},
	'en-US': {'lang': 'English (US)'},
}
delete messages['en']['lang'] // do not show plain English in the list

// not too happy with this time stuff in general
declare global {
	// kludge to work around missing property in TypeScript
	namespace Intl {
		interface DateTimeFormatOptions {
			hourCycle?: string
		}
	}
}

const dtfBase: DateTimeFormat = {
	short: {year: 'numeric', month: '2-digit', day: '2-digit'},
	long: {year: 'numeric', month: 'long', day: 'numeric'},
	yearMonth: {year: 'numeric', month: 'long'},
	weekDayMonth: {weekday: 'short', day: 'numeric', month: 'numeric'}
}
const dtf24: DateTimeFormat = {}, dtf12: DateTimeFormat = {}
Object.assign(dtf24, dtfBase)
Object.assign(dtf12, dtfBase)
dtf24.timeHMS = {hour: '2-digit', minute: '2-digit', second: '2-digit', hourCycle: 'h23'}
dtf24.timeHM = {hour: '2-digit', minute: '2-digit', hourCycle: 'h23'}
dtf24.timeH = {hour: '2-digit', minute: '2-digit', hourCycle: 'h23'}
dtf24.timeHOnly = {hour: '2-digit', hourCycle: 'h23'}
dtf12.timeHMS = {hour: '2-digit', minute: '2-digit', second: '2-digit', hourCycle: 'h12'}
dtf12.timeHM = {hour: '2-digit', minute: '2-digit', hourCycle: 'h12'}
dtf12.timeH = {hour: '2-digit', hourCycle: 'h12'}
dtf12.timeHOnly = {hour: '2-digit', hourCycle: 'h12'}

const startTime12 = readStorage('meteonook_timeFormat', (e) => e == '12') || false

// prepare start array
const dateTimeFormats: DateTimeFormats = {}
for (const k of Object.keys(messages)) {
	dateTimeFormats[k] = startTime12 ? dtf12 : dtf24
}

function matchBrowserLanguage(lang: string): string {
	const tags = lang.split('-')
	const first = tags[0]

	if (first === 'zh') {
		// special logic for Chinese
		// prefer Hans/Hant tags first, if neither exists, check for country
		if (tags.indexOf('hant') >= 0)
			return 'zh-Hant'
		else if (tags.indexOf('hans') >= 0)
			return 'zh-Hans'
		else if (tags.indexOf('tw') >= 0)
			return 'zh-TW'
		else if (tags.indexOf('hk') >= 0 || tags.indexOf('mo') >= 0)
			return 'zh-Hant'
		else
			return 'zh-Hans'
	}

	// look for an exact match (for en-GB, en-US, de-CH, etc)
	for (const [k, v] of Object.entries(messages)) {
		if (v.lang && k.toLowerCase() === lang)
			return k
	}

	// look for a language-only match (for es, de, fr, etc)
	for (const [k, v] of Object.entries(messages)) {
		if (v.lang && k.toLowerCase() === first)
			return k
	}

	// if none are found (no base language, or English variant)
	// then just default to en-GB
	return 'en-GB'
}

let startLocale = readStorage('meteonook_language', e => e)
if (startLocale === 'en')
	startLocale = (navigator.language === 'en-US') ? 'en-US' : 'en-GB'
else if (startLocale === 'zh-CN')
	startLocale = 'zh-Hans' // this file was originally called zh-CN when it was the only Chinese variant implemented
else if (startLocale === null)
	startLocale = matchBrowserLanguage(navigator.language.toLowerCase())

const i18n = new VueI18n({
	locale: startLocale,
	fallbackLocale: 'en',
	messages: Object.freeze(messages),
	dateTimeFormats,
	silentFallbackWarn: true,
})

import App from './components/App.vue'
import { readStorage } from './utils'
new Vue({
	data: {time12: startTime12},
	methods: {
		setTime12(time12: boolean) {
			this.time12 = time12
			for (const k of Object.keys(messages)) {
				this.$i18n.setDateTimeFormat(k, time12 ? dtf12 : dtf24)
			}
		}
	},
	i18n,
	render: createElement => createElement(App)
}).$mount('#app');
