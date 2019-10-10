<template>
  <div>
    <s3wf2-article :dom="result.document.blocks" :characters="result.document.characters" v-if="success" />
    <ul v-else-if="failure">
      <li v-for="(err, i) in result.errors" :key="i">{{ err.description }}</li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import S3wf2Article from './S3wf2Article.vue';
import { debounce } from 'lodash';
import { getParser } from '../bootstrap';

type AbstractNode = string | ElementNode;

interface ElementNode {
  children: AbstractNode[];
  node_type: string;
  parameters: ElementNode[];
};

interface BlockNode {
  children: AbstractNode[];
  node_type: string;
}

interface ParseError {
  line_number: number;
  description: string;
}

type ParseResult = {
  document: null;
  errors: ParseError[];
} | {
  document: {
    characters: { [c: string]: any };
    blocks: BlockNode[];
  }
  errors: null;
};

interface S3wf2RendererData {
  parser: typeof import('s3wf2') | null;
  result: ParseResult | null;
  parseDebounced: any | null;
}

export default Vue.extend({
  components: {
    S3wf2Article,
  },

  props: {
    source: {
      type: String,
      required: true,
      default: '',
    }
  },

  data(): S3wf2RendererData {
    return {
      parser: null,
      result: null,
      parseDebounced: () => null,
    };
  },

  watch: {
    source(newSource: string) {
      this.parseDebounced(newSource);
    }
  },

  computed: {
    success(): boolean {
      return this.result !== null && this.result.document !== null;
    },
    failure(): boolean {
      return this.result !== null && this.result.errors !== null;
    }
  },

  async mounted() {
    this.parser = await getParser();
    this.parseDebounced = debounce((source: string) => {
      this.result = this.parser?.parse(source);
    }, 500);
  }
});
</script>
