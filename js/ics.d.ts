import Vue from 'vue';
import ICS from 'vue-ics';

declare module 'vue/types/vue' {
  interface Vue {
    $ics: ICS;
  }
}