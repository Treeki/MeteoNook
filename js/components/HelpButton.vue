<template>
    <b-button size='sm' variant='outline-info' pill v-b-tooltip='$t(stringKey)'>ℹ️</b-button>
</template>

<script lang='ts'>
import { Vue, Component, Prop } from 'vue-property-decorator'

@Component
export default class HelpButton extends Vue {
	@Prop(String) readonly stringKey!: string
}
</script>

