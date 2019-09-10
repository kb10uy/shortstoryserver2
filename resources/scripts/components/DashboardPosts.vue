<template>
<div class="dashboard-posts">
  <transition-group name="items" tag="div">
    <post-item class="items-item" v-for="post of posts" :key="post.id" :post="post" />
  </transition-group>
  <spinner :loading="loading" />
</div>
</template>

<style lang="scss" scoped>
.items-item {
  transition: 0.3s opacity;
}

.items-enter, .items-leave-to {
  opacity: 0;
}

.items-leave-active {
  position: absolute;
}
</style>

<script lang="ts">
import { kbs3 } from '../bootstrap';
import Vue from 'vue';
import Spinner from './Spinner.vue';
import PostItem from './PostItem.vue';

interface DashboardPostsData {
  posts: any[] | false;
  loading: boolean;
}

export default Vue.extend({
  components: {
    PostItem,
    Spinner,
  },

  props: {
    userId: {
      type: String,
      required: true,
    },
  },

  data(): DashboardPostsData {
    return {
      posts: false,
      loading: false,
    };
  },

  computed: {
    hasMore(): boolean {
      // TODO: なんかやる
      return false;
    },
  },

  async mounted() {
    await this.loadPosts();
  },

  methods: {
    async loadPosts() {
      this.loading = true;
      this.posts = (await kbs3.get(`/api/users/latest_user_posts`)).data;
      this.loading = false;
    },
  },
});
</script>
