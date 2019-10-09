import axios from 'axios';

let parserModule: typeof import('s3wf2') | undefined = undefined;

/** kbs3 API にリクエストを飛ばす用の axios instance */
export const kbs3 = axios.create();
kbs3.defaults.headers.common['X-Requested-With'] = 'XMLHttpRequest';

const token = document.head.querySelector('meta[name="csrf-token"]');
if (token) {
  kbs3.defaults.headers.common['X-CSRF-TOKEN'] = (token as HTMLMetaElement).content;
} else {
  console.error('CSRF token not found: https://laravel.com/docs/csrf#csrf-x-csrf-token');
}

/**
 * なんと WASM で実装されているパーサーを取得する。
 */
export async function getParser(): Promise<typeof import('s3wf2')> {
  if (!parserModule) {
    parserModule = await import('s3wf2');
  }

  return parserModule;
}

declare global {
  interface Window {
    getParser: any;
  }
}
window.getParser = getParser;
