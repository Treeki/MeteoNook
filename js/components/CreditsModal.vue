<template>
	<b-modal :id='id' :title="$t('credTitle')" scrollable hide-footer>
		<i18n tag='p' path='credSource'>
			<template v-slot:license>
				<a href='https://www.gnu.org/licenses/agpl-3.0.en.html'>GNU Affero General Public License 3.0</a>
			</template>
			<template v-slot:repo>
				<a href='https://github.com/Treeki/MeteoNook'>github.com/Treeki/MeteoNook</a>
			</template>
		</i18n>
		<p class='font-weight-bold'>{{ $t('credThank1') }}</p>
		<ul>
			<li>
				<b>{{ $t('credThank1a') }}</b>
				Ahri, Alexis, andrewtabs, astro, capstone, ctar17, Denna, GoldenCrater, rainbowannie, robotic_scarab
			</li>
			<li>
				<b>{{ $t('credThank1b') }}</b>
				<a href='https://twitter.com/jerdieh'>@jerdieh</a> <i>(español)</i>,
				<a href='https://github.com/TheVaan'>@TheVaan</a> <i>(Deutsch)</i>,
				<a href='https://github.com/einfallstoll'>@einfallstoll</a> <i>(Deutsch [CH])</i>,
				Thordgar (Italiano)</i>,
				<a href='https://github.com/Swpolo'>@Swpolo</a> <i>(Français)</i>,
				<a href='https://github.com/IceLitty'>@IceLitty</a> <i>(简体中文)</i>,
				<a href='https://github.com/Small-Ku'>@Small-Ku</a> <i>(繁體中文)</i>,
				Laurence Chen <i>(臺灣正體)</i>,
				<a href='https://github.com/yuma-m'>@yuma-m</a>,
				<a href='https://github.com/Starwort'>@Starwort</a> <i>(日本語)</i>,
				<a href='https://twitter.com/yozhique_acnh'>@yozhique_acnh</a> <i>(Русский)</i>,
				Like_it, Kuiprux (한국어), Angela, Mees Muller, Bjørn, Thijsvb <i>(Nederlands)</i>,
				RadarCZ <i>(Česky)</i>
			</li>
		</ul>
		<p class='font-weight-bold'>{{ $t('credThank2') }}</p>
		<ul>
			<li><a href='https://getbootstrap.com'>Bootstrap</a></li>
			<li><a href='https://bootstrap-vue.org'>BootstrapVue</a></li>
			<li><a href='https://www.rust-lang.org'>Rust</a></li>
		</ul>
		<p class='font-weight-bold'>{{ $t('credThank3') }}</p>
		<ul>
			<li><a href='https://github.com/zeldamods/byml-v2'>byml-v2</a></li>
		</ul>
	</b-modal>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'

@Component
export default class CreditsModal extends Vue {
	@Prop(String) readonly id!: string
}
</script>


