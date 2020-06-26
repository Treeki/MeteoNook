import Vue from 'vue'
import { ButtonPlugin, CalendarPlugin, DropdownPlugin, FormCheckboxPlugin, FormRadioPlugin, LayoutPlugin, NavbarPlugin, TabsPlugin, FormGroupPlugin, ModalPlugin, FormSelectPlugin, InputGroupPlugin, LinkPlugin, SpinnerPlugin, ProgressPlugin } from 'bootstrap-vue'
import VueI18n, { DateTimeFormats, DateTimeFormat } from 'vue-i18n'

Vue.use(ButtonPlugin)
Vue.use(CalendarPlugin)
Vue.use(DropdownPlugin)
Vue.use(FormCheckboxPlugin)
Vue.use(FormGroupPlugin)
Vue.use(FormRadioPlugin)
Vue.use(FormSelectPlugin)
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

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

const root = document.createElement('div')
root.id = 'app'
document.body.appendChild(root)

import messages from './translations'

// not too happy with this
const dtfBase: DateTimeFormat = {
	short: {year: 'numeric', month: '2-digit', day: '2-digit'},
	long: {year: 'numeric', month: 'long', 'day': 'numeric'},
	yearMonth: {year: 'numeric', month: 'long'},
}
const dtf24: DateTimeFormat = {}, dtf12: DateTimeFormat = {}
Object.assign(dtf24, dtfBase)
Object.assign(dtf12, dtfBase)
dtf24.timeHMS = {hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false}
dtf24.timeHM = {hour: '2-digit', minute: '2-digit', hour12: false}
dtf24.timeH = {hour: '2-digit', minute: '2-digit', hour12: false}
dtf24.timeHOnly = {hour: '2-digit', hour12: false}
dtf12.timeHMS = {hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true}
dtf12.timeHM = {hour: '2-digit', minute: '2-digit', hour12: true}
dtf12.timeH = {hour: '2-digit', hour12: true}
dtf12.timeHOnly = {hour: '2-digit', hour12: true}

// prepare start array
const dateTimeFormats: DateTimeFormats = {}
for (const k of Object.keys(messages)) {
	dateTimeFormats[k] = dtf24
}

const i18n = new VueI18n({
	locale: 'en-GB',
	fallbackLocale: 'en',
	messages,
	dateTimeFormats,
	silentFallbackWarn: true,
})

import App from './components/App.vue'
new Vue({
	data: {time12: false},
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
