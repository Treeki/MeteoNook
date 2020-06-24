import Vue from 'vue'
import { ButtonPlugin, CalendarPlugin, DropdownPlugin, FormCheckboxPlugin, FormRadioPlugin, LayoutPlugin, NavbarPlugin, TabsPlugin, FormGroupPlugin, ModalPlugin, FormSelectPlugin, InputGroupPlugin, LinkPlugin } from 'bootstrap-vue'
import VueI18n, { DateTimeFormats } from 'vue-i18n'

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
Vue.use(TabsPlugin)
Vue.use(VueI18n)

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

const root = document.createElement('div')
root.id = 'app'
document.body.appendChild(root)

import messages from './translations'

const dateTimeFormats: DateTimeFormats = {}
for (const k of Object.keys(messages)) {
	dateTimeFormats[k] = {
		short: {year: 'numeric', month: '2-digit', day: '2-digit'},
		yearMonth: {year: 'numeric', month: 'long'}
	}
}

const i18n = new VueI18n({
	locale: 'en-GB',
	fallbackLocale: 'en',
	messages,
	dateTimeFormats
})

import App from './components/App.vue'
new Vue({ i18n, render: createElement => createElement(App) }).$mount('#app');
