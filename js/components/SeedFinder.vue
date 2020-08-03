<template>
	<div>
		<p class='mt-3'>{{ $t('sIntro') }}</p>
		<h4 class='mt-4'>{{ $t('sTTypes') }}</h4>
		<div v-html="timeify($t('sTypes1'))"></div>
		<div class='float-md-right img-thumbnail wtExplainerImgHolder mb-2'>
			<img src='explainer.jpg' class='wtExplainerImg'>
			<span class='wte0'>{{ $t('lstPatternNoSnow0') }}</span>
			<span class='wte1'>{{ $t('lstPatternNoSnow1') }}</span>
			<span class='wte2'>{{ $t('lstPatternNoSnow2') }}</span>
			<span class='wte3'>{{ $t('lstPatternNoSnow3') }}</span>
			<span class='wte4'>{{ $t('lstPatternNoSnow4') }}</span>
			<span class='wte5'>{{ $t('lstPatternNoSnow5') }}</span>
		</div>
		<div v-html="timeify($t('sTypes2'))"></div>

		<h4>{{ $t('spcHeader') }} {{ $t('newSuffix') }}</h4>
		<div v-html="$t('spcBlurb')"></div>
		<b-card no-body>
			<b-tabs card>
				<b-tab :title="$t('lstSpecialCloudsLevels1')">
					<p v-html='cumulonimbusDates'></p>
					<p>{{ cumulonimbusBlurb }}</p>
					<p>
						{{ $t('spcExamplePrompt') }}
						<b-button variant='outline-secondary' size='sm' href='example_cumulonimbus_clear.jpg' target='_blank'>
							{{ $t('lstPatternChoices0') }}
						</b-button>
						<b-button variant='outline-secondary' size='sm' href='example_cumulonimbus_sunny.jpg' target='_blank'>
							{{ $t('lstPatternChoices1') }}
						</b-button>
					</p>
				</b-tab>
				<b-tab :title="$t('lstSpecialCloudsLevels2')">
					<p v-html='cirrusDates'></p>
					<p>{{ $t('spcCirrus') }}</p>
					<p>{{ cirrusBlurb }}</p>
					<p>
						{{ $t('spcExamplePrompt') }}
						<b-button variant='outline-secondary' size='sm' href='example_cirrus_clear.jpg' target='_blank'>
							{{ $t('lstSpecialClouds1') }} ({{ $t('lstPatternChoices0') }})
						</b-button>
						<b-button variant='outline-secondary' size='sm' href='example_cirrus_sunny.jpg' target='_blank'>
							{{ $t('lstSpecialClouds1') }} ({{ $t('lstPatternChoices1') }})
						</b-button>
						<b-button variant='outline-secondary' size='sm' href='example_cirrocumulus_clear.jpg' target='_blank'>
							{{ $t('lstSpecialClouds2') }} ({{ $t('lstPatternChoices0') }})
						</b-button>
						<b-button variant='outline-secondary' size='sm' href='example_cirrocumulus_sunny.jpg' target='_blank'>
							{{ $t('lstSpecialClouds2') }} ({{ $t('lstPatternChoices1') }})
						</b-button>
					</p>
				</b-tab>
				<b-tab :title="$t('lstSpecialCloudsLevels4')">
					<p v-html='billowDates'></p>
					<p>{{ billowBlurb }}</p>
					<p>{{ $t('spcDifficultWarning') }}</p>
					<p>
						{{ $t('spcExamplePrompt') }}
						<b-button variant='outline-secondary' size='sm' href='example_billow_clear.jpg' target='_blank'>
							{{ $t('lstPatternChoices0') }}
						</b-button>
						<b-button variant='outline-secondary' size='sm' href='example_billow_sunny.jpg' target='_blank'>
							{{ $t('lstPatternChoices1') }}
						</b-button>
					</p>
				</b-tab>
				<b-tab :title="$t('lstSpecialCloudsLevels3')">
					<p v-html='thinDates'></p>
					<p>{{ thinBlurb }}</p>
					<p>
						{{ $t('spcExamplePrompt') }}
						<b-button variant='outline-secondary' size='sm' href='example_thin_clear.jpg' target='_blank'>
							{{ $t('lstPatternChoices0') }}
						</b-button>
						<b-button variant='outline-secondary' size='sm' href='example_thin_sunny.jpg' target='_blank'>
							{{ $t('lstPatternChoices1') }}
						</b-button>
					</p>
				</b-tab>
			</b-tabs>
		</b-card>

		<h4 class='mt-4'>{{ $t('sTStars') }}</h4>
		<div v-html="timeify($t('sStars'))"></div>
		<h4 class='mt-4'>{{ $t('sTTips') }}</h4>
		<div v-html="timeify($t('sTips'))"></div>

		<h4 class='mt-4'>{{ $t('tStep1') }}</h4>
		<b-form-radio-group class='mb-3' stacked id='seedFinderHemisphere' v-model.number='hemisphere'>
			<b-form-radio value='0'>{{ $t('tNorthernHemi') }}</b-form-radio>
			<b-form-radio value='1'>{{ $t('tSouthernHemi') }}</b-form-radio>
		</b-form-radio-group>

		<p>
			{{ $t('tResetBlurb') }}
			<b-button size='sm' variant='outline-secondary' @click='resetData'>
				{{ $t('tResetButton') }}
			</b-button>
		</p>

		<p>
			{{ $t('tExportBlurb') }}
			<b-button size='sm' variant='outline-secondary' @click='exportData'>
				{{ $t('tExportButton') }}
			</b-button>
			<b-modal :title="$t('deTitle')" hide-footer id='exportDataModal'>
				<b-form-textarea ref='exportedDataTextarea' :value='exportedData' readonly rows='6' @click='selectAllExportData'></b-form-textarea>
			</b-modal>
		</p>

		<div class='d-sm-flex'>
			<b-calendar
				class='align-self-start mr-3'
				min='2000-01-01' max='2060-12-31'
				label-help=''
				:locale='$i18n.locale'
				v-model='selectedDay'
				:date-info-fn='dateClass'
				value-as-date
				></b-calendar>
			<day-editor
				class='flex-grow-1'
				:day='getDay(selectedDay)'
				:hemisphere='hemisphere'
				@show-day="showDayModal"
			>
			</day-editor>
		</div>

		<h4 class='mt-4'>{{ $t('tStep2') }}</h4>
		<guess-worker-view
			:days='days'
			:hemisphere='hemisphere'
			@preview-seed='previewSeed'
			>
		</guess-worker-view>
	</div>
</template>

<script lang='ts'>
import { Vue, Component, Watch } from 'vue-property-decorator'
import DayEditor from './DayEditor.vue'
import GuessWorkerView from './GuessWorkerView.vue'
import {createDayInfo, DayInfo, DayType, ShowerType, Hemisphere, isDayNonEmpty, DayForecast} from '../model'
import {readStorageObject, readStorage, writeStorageObject, writeStorage, makeTime, makeDayMonth} from '../utils'
import { BFormTextarea } from 'bootstrap-vue'

export type SeedFinderDays = {[key: string]: DayInfo}

function updateSFDays(days: SeedFinderDays) {
	for (const day of Object.values(days)) {
		for (const type of day.types) {
			if (type.specialCloud === undefined)
				type.specialCloud = null
		}
	}
}

const sfDaysKey = 'meteonook_sf_days'
const sfHemisphereKey = 'meteonook_sf_hemisphere'
function loadSFDays(): SeedFinderDays {
	let obj = readStorageObject<SeedFinderDays>(sfDaysKey)
	if (obj !== null) {
		updateSFDays(obj)
		return obj
	}

	obj = readStorageObject<SeedFinderDays>('meteonook_data', (data: any) => data.version === 2)
	if (obj !== null) {
		delete obj.version
		updateSFDays(obj)
		return obj
	}

	return {}
}

function loadSFHemisphere(): Hemisphere {
	let hemi = readStorage(sfHemisphereKey, blob => (blob == 'n') ? Hemisphere.Northern : Hemisphere.Southern)
	if (hemi !== null)
		return hemi
	else
		return Hemisphere.Northern
}

function saveSFDays(days: SeedFinderDays) {
	writeStorageObject(sfDaysKey, days)
}
function saveSFHemisphere(hemisphere: Hemisphere) {
	writeStorage(sfHemisphereKey, (hemisphere == Hemisphere.Northern) ? 'n' : 's')
}



@Component({components: {DayEditor, GuessWorkerView}})
export default class SeedFinder extends Vue {
	selectedDay = new Date()
	days: SeedFinderDays = loadSFDays()
	hemisphere = loadSFHemisphere()

	$refs!: {
		exportedDataTextarea: BFormTextarea
	}

	getDay(date: Date) {
		const key = `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()}`
		if (this.days[key] === undefined)
			Vue.set(this.days, key, createDayInfo(date))
		return this.days[key]
	}
	
	resetData() {
		this.days = {}
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

	dateClass(ymd: string, date: Date): string {
		const key = `${date.getFullYear()}-${date.getMonth()+1}-${date.getDate()}`
		const day = this.days[key]
		if (day !== undefined && isDayNonEmpty(day))
			return 'table-info'
		return ''
	}

	@Watch('hemisphere')
	onHemisphereChanged() { saveSFHemisphere(this.hemisphere) }
	@Watch('days', {deep: true})
	onDaysChanged() { saveSFDays(this.days) }

	previewSeed(seed: number, multiFlag: boolean) {
		this.$emit('preview-seed', seed, this.hemisphere, multiFlag)
	}
	showDayModal(day: DayForecast) {
		this.$emit('show-day', day)
	}


	exportedData = ''
	exportData() {
		this.$bvModal.show('exportDataModal')
		this.exportedData = JSON.stringify({days: this.days, hemisphere: this.hemisphere})
	}
	selectAllExportData() {
		const textarea = this.$refs.exportedDataTextarea.$el as HTMLTextAreaElement
		textarea.setSelectionRange(0, textarea.value.length)
	}


	makeSpecialCloudDateText(startN: Date, endN: Date, startS: Date, endS: Date) {
		return this.$t('spcDates', {
			startN: this.$d(startN, 'monthDay'),
			endN: this.$d(endN, 'monthDay'),
			startS: this.$d(startS, 'monthDay'),
			endS: this.$d(endS, 'monthDay')
		})
	}
	makeSpecialCloudBlurb(certainty: string, startHour: number, endHour: number, startThreshold: string, endThreshold: string, groupNote: boolean) {
		const appearTime = this.$t(`spc${certainty}AppearTime`, {
			startTime: this.$d(makeTime(startHour, 0), 'timeH'),
			endTime: this.$d(makeTime(endHour, 0), 'timeH')
		})
		const fadeNote = this.$t('spcFadeNote', {start: startThreshold, end: endThreshold})
		return [appearTime, groupNote ? this.$t('spcGroupNote') : '', fadeNote].join(' ')
	}
	get cumulonimbusDates() {
		return this.makeSpecialCloudDateText(
			makeDayMonth(21, 7), makeDayMonth(15, 9),
			makeDayMonth(21, 1), makeDayMonth(15, 3)
		)
	}
	get cumulonimbusBlurb() {
		return this.makeSpecialCloudBlurb('Certain', 9, 21, '5', '5', false)
	}
	get cirrusDates() {
		return this.makeSpecialCloudDateText(
			makeDayMonth(16, 9), makeDayMonth(30, 11),
			makeDayMonth(16, 3), makeDayMonth(31, 5)
		)
	}
	get cirrusBlurb() {
		return this.makeSpecialCloudBlurb('Uncertain', 6, 4, '10', '10', true)
	}
	get billowDates() {
		return this.makeSpecialCloudDateText(
			makeDayMonth(1, 12), makeDayMonth(29, 2),
			makeDayMonth(1, 6), makeDayMonth(31, 8)
		)
	}
	get billowBlurb() {
		return this.makeSpecialCloudBlurb('Uncertain', 6, 17, '10', '15', true)
	}
	get thinDates() {
		return this.makeSpecialCloudDateText(
			makeDayMonth(1, 3), makeDayMonth(31, 5),
			makeDayMonth(1, 9), makeDayMonth(30, 11)
		)
	}
	get thinBlurb() {
		return this.makeSpecialCloudBlurb('Uncertain', 6, 4, '5', '10', true)
	}
}
</script>

<style lang='scss'>
.progress-bar { transition: none; }

.wtExplainerImgHolder {
	width: 45%;
	max-width: 720px;
	margin-left: .5rem;
	position: relative;
	& > span {
		width: 50%;
		position: absolute;
		text-align: center;
		color: white;
		text-shadow: -1px -1px 1px #494867, 1px -1px 1px #494867, -1px 1px 1px #494867, 1px 1px 1px #494867, 0 3px 5px black;
		font-weight: bold;
	}
}
.wte0,.wte2,.wte4 { left: 0; }
.wte1,.wte3,.wte5 { left: 50%; }
.wte0,.wte1 { bottom: calc(66.67% + .167rem); }
.wte2,.wte3 { bottom: calc(33.33% + .333rem); }
.wte4,.wte5 { bottom: .5rem; }
.wtExplainerImg {
	width: 100%;
}
@media (max-width: 767.98px) {
	.wtExplainerImgHolder {
		width: 80%; max-width: 400px;
		display: block;
		margin-left: auto;
		margin-right: auto;
	}
}
</style>
