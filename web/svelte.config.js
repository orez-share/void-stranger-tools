import adapterAuto from '@sveltejs/adapter-auto';
import adapterStatic from '@sveltejs/adapter-static';

const isRelease = process.env.NODE_ENV === "production";
const baseUrl = process.env.BASE_PATH;

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
    // If your environment is not supported or you settled on a specific environment, switch out the adapter.
    // See https://kit.svelte.dev/docs/adapters for more information about adapters.
    adapter: isRelease ? adapterStatic({
      fallback: "index.html",
    }) : adapterAuto(),
    paths: {
      base: baseUrl,
    },
  }
};

export default config;
