<template>
  <div>
    <input type="text" name="composition" :placeholder="placeholder" v-model="tagText" @keypress.enter.prevent="addTag">
    <input type="hidden" name="tags_json" id="tags_json">
    <ul class="tags">
      <li class="tag" v-for="(tag, index) of tags" :key="index">
        {{ tag }}
        <a href="#" @click.prevent="deleteTag(index)"><i class="fas fa-times"></i></a>
      </li>
    </ul>
  </div>
</template>

<style lang="scss" scoped>
ul.tags {
  display: flex;
  grid-row: 3;
  grid-column: 1;
  padding: 0;
  margin: 0;
  margin-top: 10px;
  gap: 10px;
}
</style>

<script lang="ts">
import Vue from 'vue';
export default Vue.extend({
  props: {
    // 入力欄のプレースホルダー
    placeholder: {
      type: String,
      required: true,
    },
  },

  data() {
    return {
      // 設定されているタグ
      tags: [] as string[],

      // 入力中のタグ
      tagText: '',
    };
  },

  computed: {
    tagsJson(): string {
      return JSON.stringify(this.tags);
    },
  },

  methods: {
    // タグを追加
    addTag() {
      const normalized = this.tagText.trim();
      if (normalized === '') return;

      if (!this.tags.includes(normalized)) {
        this.tags.push(normalized);
      }
      this.tagText = '';
    },

    // 指定インデックスのタグを削除
    deleteTag(index: number) {
      this.tags.splice(index, 1);
    },
  },
});
</script>

