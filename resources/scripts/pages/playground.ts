import Vue from 'vue';
import S3wf2Renderer from './../components/S3wf2Renderer.vue';

const app = new Vue({
  components: {
    S3wf2Renderer,
  },

  data() {
    return {
      source: '',
    };
  },
});

app.$mount('#app');
