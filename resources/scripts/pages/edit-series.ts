import Vue from 'vue';
import Draggable from 'vuedraggable';
import { kbs3 } from '../bootstrap';

const app = new Vue({
  components: {
    Draggable,
  },

  data() {
    return {
      posts: [] as any[],
    };
  },

  async mounted() {
    const match = /\/series\/(\d+)\/edit_order$/.exec(location.href);
    if (!match) return;
    this.posts = (await kbs3.get(`/api/series/list_posts?series_id=${match[1]}`)).data;
  },
});

app.$mount('#app');
