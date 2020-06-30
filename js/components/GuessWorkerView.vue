<template>
	<div>
		<div class='d-flex'>
			<b-button variant='primary' :disabled='running' @click='startSearch'>
				<b-spinner small v-show='running'></b-spinner>
				{{ $t(running ? 'tWorking' : 'tStart') }}
			</b-button>
			<b-button variant='primary' class='ml-2' :disabled='!running' @click='stopSearch'>
				{{ $t('tCancel') }}
			</b-button>
		</div>
		<p class='my-2' v-html='infoText' @click='handleInfoClick'></p>
		<b-progress :value='progress' max='100' show-progress></b-progress>
	</div>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'
import { Guesser, GuessData, Hemisphere, GuesserResult } from '../../pkg'
import { populateGuessData, DayInfo, PopulateErrorKind } from '../model'
import { TranslateResult } from 'vue-i18n'
import { SeedFinderDays } from './SeedFinder.vue'

@Component
export default class GuessWorkerView extends Vue {
	@Prop(Object) readonly days!: SeedFinderDays
	@Prop(Number) readonly hemisphere!: Hemisphere

	running: boolean = false
	progress: number = 0
	results: number[] = []
	timeout: number|null = null
	lastResult: GuesserResult|null = null
	guesser: Guesser|null = null
	guessData: GuessData|null = null
	errorText: TranslateResult|null = null

	beforeDestroy() {
		this.stopSearch()
	}


	startSearch() {
		this.stopSearch()

		this.guessData = GuessData.new(this.hemisphere)
		for (const day of Object.values(this.days)) {
			const error = populateGuessData(this.hemisphere, this.guessData, day)
			if (error !== undefined) {
				this.stopSearch() // free the GuessData
				const date = new Date(day.y, day.m - 1, day.d, error.hour||0, error.minute||0)
				switch (error.kind) {
					case PopulateErrorKind.NoPatterns:
						this.errorText = this.$t('tErrNoPatterns', {date: this.$d(date, 'long')})
						break
					case PopulateErrorKind.StarConflict:
						this.errorText = this.$t('tErrStarConflict', {date: this.$d(date, 'long'), time: this.$d(date, 'timeHM')})
						break
				}
				return
			}
		}

		this.guesser = Guesser.new(0, 0x7fffffff)
		this.running = true
		this.progress = 0
		this.results = []
		this.timeout = window.setTimeout(() => this.work(), 0)
	}

	stopSearch() {
		this.running = false
		this.errorText = null
		if (this.timeout !== null) {
			clearTimeout(this.timeout)
			this.timeout = null
		}
		if (this.guesser !== null) {
			this.guesser.free()
			this.guesser = null
		}
		if (this.guessData !== null) {
			this.guessData.free()
			this.guessData = null
		}
	}

	work() {
		if (this.guesser !== null && this.guessData !== null) {
			const result = this.guesser.work(this.guessData, 0x10000)
			this.progress = this.guesser.getPercentage()

			const resultCount = this.guesser.getResultCount()
			if (resultCount != this.results.length) {
				for (let i = this.results.length; i < resultCount; i++) {
					this.results.push(this.guesser.getResult(i))
				}
			}

			this.lastResult = result
			if (result == GuesserResult.Incomplete) {
				this.timeout = window.setTimeout(() => this.work(), 0)
			} else {
				this.stopSearch()

				// did we find a seed?
				// if yes, throw it over to the Overview anyway
				if (result == GuesserResult.Complete && this.results.length == 1) {
					this.$emit('preview-seed', this.results[0], false)
				}
			}
		}
	}

	makeSeedLink(seed: number): string {
		// this is a really nasty kludge
		// oh well :p
		return `<a href='#' data-seed='${seed}'>${seed}</a>`
	}
	handleInfoClick(event: MouseEvent) {
		if (event.target !== null) {
			const target = event.target as HTMLElement
			if (target.dataset.seed !== undefined) {
				const seed = parseInt(target.dataset.seed, 10)
				this.$emit('preview-seed', seed, true)
				event.preventDefault()
				event.stopPropagation()
			}
		}
	}

	get infoText(): TranslateResult {
		if (this.errorText !== null) {
			return this.errorText
		} else if (this.lastResult === GuesserResult.Failed) {
			return this.$t('tInfoMoreData')
		} else if (this.lastResult === GuesserResult.Incomplete) {
			const count = this.results.length
			if (count > 0)
				return this.$t('tInfoRunningC', {count, seeds: this.results.join(', ')})
			else
				return this.$t('tInfoRunning')
		} else if (this.lastResult == GuesserResult.Complete) {
			const count = this.results.length
			if (count > 1) {
				const links = this.results.map(this.makeSeedLink).join(', ')
				return this.$t('tInfoMultiSeed', {count, seeds: links})
			} else if (count == 1)
				return this.$t('tInfoSeed', {seed: this.results[0]})
			else
				return this.$t('tInfoNoSeeds')
		} else {
			return ''
		}
	}
}
</script>


