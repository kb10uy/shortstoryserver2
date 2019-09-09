import Vue from 'vue';
import ModalDialog from './../components/ModalDialog.vue';

const app = new Vue({
  components: {
    ModalDialog,
  },

  data() {
    return {
      shown: {
        series: false,
        bookmark: false,
      },
    };
  },

  methods: {
    async showSeriesDialog() {
      this.shown.series = true;
    },

    async addToSeries() {
      this.shown.series = false;
    },
  },
});

app.$mount('#app');
