import Vue from 'vue';
import Draggable from 'vuedraggable';
import { kbs3 } from '../bootstrap';

const app = new Vue({
  components: {
    Draggable,
  },

  data() {
    return {
      seriesId: -1,
      sending: false,
      posts: [] as any[],
    };
  },

  async mounted() {
    const match = /\/series\/(\d+)\/edit_order$/.exec(location.href);
    if (!match) return;
    this.seriesId = Number(match[1]);
    this.posts = (await kbs3.get(`/api/series/list_posts?series_id=${this.seriesId}`)).data;

    for (const post of this.posts) {
      post.willBeDeleted = false;
    }
  },

  methods: {
    toggleDeletion(index: number): void {
      Vue.set(this.posts, index, {
        ...this.posts[index],
        willBeDeleted: !this.posts[index].willBeDeleted,
      });
    },

    async sendData() {
      this.sending = true;
      const data = this.posts.map((post) => ({
        post_id: post.id,
        remove: post.willBeDeleted,
      }));

      try {
        await kbs3.post('/api/series/update', {
          series_id: this.seriesId,
          data,
        });
        alert('更新しました。');
        location.href = `/series/${this.seriesId}`;
      } catch(e) {
        alert('更新できませんでした。');
        this.sending = false;
      }
    },
  },
});

app.$mount('#app');
