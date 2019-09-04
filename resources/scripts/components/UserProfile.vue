<template>
<div class="user-profile">
  <div class="avatar-box">
    <div class="avatar-content">
      <img :src="avatar" alt="avatar">
      <h2>{{ name }}</h2>
    </div>
  </div>
  <div class="counts-box">
    <div class="count activated">
      <span id="user-profile-posts" class="number"><slot name="posts-count" /></span>
      <span class="label" for="user-profile-posts">投稿</span>
    </div>
    <div class="count">
      <span id="user-profile-series" class="number"><slot name="series-count" /></span>
      <span class="label" for="user-profile-series">シリーズ</span>
    </div>
    <div class="count">
      <span id="user-profile-bookmarks" class="number"><slot name="bookmarks-count" /></span>
      <span class="label" for="user-profile-bookmarks">ブックマーク</span>
    </div>
  </div>

  <transition-group name="items" tag="div">
    <post-item class="items-item" v-for="post of posts" :key="post.id" :post="post" />
  </transition-group>
  <spinner :loading="loading" />
</div>
</template>

<style lang="scss" scoped>
@import '../../styles/variables';

.user-profile {
  .avatar-box {
    display: grid;
    grid-template-columns: 1fr minmax(100px, 400px) 1fr;
    grid-template-rows: max-content;
    background-color: $floating-color-bg;
    border: 1px solid $floating-color-br;
    border-radius: 10px 10px 0 0;

    .avatar-content {
      grid-column: 2;
      text-align: center;

      img {
        max-height: 300px;
        border-radius: 50%;
      }

      h2 {
        margin-top: 10px;
        font-size: 1.6rem;
        font-weight: bold;
      }
    }
  }

  .counts-box {
    display: flex;
    color: $floating-color-fg;
    background-color: $floating-color-bg;
    border: 1px solid $floating-color-br;
    border-top: 0;
    cursor: pointer;

    .count {
      flex: 1;
      display: grid;
      position: relative;
      grid-template-rows: max-content max-content;
      text-align: center;
      transition: 0.2s background-color;

      &.activated::after {
        position: absolute;
        display: block;
        width: 100%;
        height: 5px;
        bottom: 0;
        background-color: $tag-color-bg;
        content: '';
      }

      span.number {
        grid-row: 1;
        font-size: 1.3rem;
        font-weight: bold;
      }

      span.label {
        grid-row: 2;
        margin-bottom: 5px;
      }
    }
  }
}

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

interface UserProfileData {
  posts: any[] | false;
  series: any[] | false;
  bookmarks: any[] | false;
  loading: boolean;
}

export default Vue.extend({
  components: {
    PostItem,
    Spinner,
  },

  props: {
    name: {
      type: String,
      required: true,
    },
    userId: {
      type: String,
      required: true,
    },
    avatar: {
      type: String,
      required: true,
    },
  },

  data(): UserProfileData {
    return {
      posts: false,
      series: false,
      bookmarks: false,
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
      this.posts = (await kbs3.get(`/api/users/latest_posts?user_id=${this.userId}`)).data;
      this.loading = false;
    },
  },
});
</script>
