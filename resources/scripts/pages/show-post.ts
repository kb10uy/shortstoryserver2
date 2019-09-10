import Vue from 'vue';
import ModalDialog from '../components/ModalDialog.vue';
import { kbs3 } from '../bootstrap';

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

      selectedSeries: -1,
      series: [] as any,
    };
  },

  methods: {
    async showSeriesDialog() {
      // TODO: ここページングしないからバック側にmax_id用意する意味ねえじゃん
      this.series = (await kbs3.get('/api/users/latest_user_series')).data;
      this.shown.series = true;
    },

    async addToSeries() {
      if (this.selectedSeries === -1) return;
      this.shown.series = false;
    },
  },
});

app.$mount('#app');
