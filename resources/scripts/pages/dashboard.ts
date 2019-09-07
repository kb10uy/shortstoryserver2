import Vue from 'vue';
import DashboardPosts from './../components/DashboardPosts.vue';

const app = new Vue({
  components: {
    DashboardPosts,
  },
});

app.$mount('#app');
