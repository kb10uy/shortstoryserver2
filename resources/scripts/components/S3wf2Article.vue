<script lang="ts">
import Vue, { CreateElement, VNode } from 'vue';

export default Vue.extend({
  props: {
    dom: {
      type: Array,
      required: true,
    },
    characters: {
      type: Object,
      required: true,
    },
  },

  render(h: CreateElement): VNode {
    const nodes = this.dom.map((b: any) => this.renderBlock(h, b));
    console.log(nodes);
    return h('article', { class: { post: true } }, nodes);
  },

  methods: {
    renderBlock(h: CreateElement, block: any): VNode {
      let tag = 'p';
      switch (block.node_type) {
        case 'paragraph':
          tag = 'p';
          break;
        case 'section':
          tag = 'h2';
          break;
        case 'subsection':
          tag = 'h3';
          break;
        case 'hori':
          tag = 'hr';
          break;
        case 'list':
          tag = 'ul';
          break;
        case 'quotation':
          tag = 'blockquote';
          break;
      }

      return h(tag, {}, block.children.map((e: any) => this.renderElement(h, e)));
    },

    renderElement(h: CreateElement, element: any): VNode | string {
      if (typeof element === 'string') return element;
      const elementMapper = (e: any) => typeof e === 'string' ? e : this.renderElement(h, e);

      switch (element.node_type) {
        case 'bold':
          return h('strong', {}, element.children.map(elementMapper));
        case 'italic':
          return h('i', {}, element.children.map(elementMapper));
        case 'underline':
          return h('span', { class: { underline: true } }, element.children.map(elementMapper));
        case 'strikethrough':
          return h('del', {}, element.children.map(elementMapper));
        case 'dots':
          return h('code', {}, element.children.map(elementMapper));
        case 'monospace':
          return h('code', {}, element.children.map(elementMapper));
        case 'link':
          return h('a', { attrs: { href: element.parameters[0].children[0] } }, element.children.map(elementMapper));
          break;
        case 'ruby': {
          const elements = element.children.map(elementMapper);
          const rubyText = element.parameters[0].children.map(elementMapper);
          elements.push(h('rp', '('));
          elements.push(h('rt', {}, rubyText));
          elements.push(h('rp', ')'));
          return h('ruby', {}, elements);
        }
        case 'item':
          return h('li', {}, element.children.map(elementMapper));
        case 'newline':
          return h('br');
        case 'line': {
          const options: any = { class: {} };
          options.class.line = true;
          options.class.inline = element.inline ?? false;
          const character = this.characters[element.character_id];
          if (character.color.startsWith('#')) {
            options.style = { color: character.color };
          } else {
            options.class[character.color] = true;
          }
          const elements = [character.display_name, ...element.children.map(elementMapper)];
          return h('span', options, elements);
        }
        default:
          return h('span');
      }
    },
  },
});
</script>
