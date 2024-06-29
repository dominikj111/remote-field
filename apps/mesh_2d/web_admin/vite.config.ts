import { defineConfig } from "vite";
import nunjucks from "nunjucks";
import { minify } from "html-minifier-terser";

const env = nunjucks.configure("./src/templates", { autoescape: true });

function nunjucksPlugin() {
    return {
        name: "vite-plugin-nunjucks",
        transformIndexHtml(html) {
            return minify(env.renderString(html, { mode: process.env.NODE_ENV }), {
                removeComments: true,
                collapseWhitespace: true,
                collapseBooleanAttributes: true,
                removeAttributeQuotes: false,
                removeEmptyAttributes: true,
            });
        },
    };
}

export default defineConfig({
    plugins: [
        nunjucksPlugin()
    ],
    build: {
        outDir: 'dist',
        rollupOptions: {
            input: {
                index: './index.html',
                style_guide: './style_guide.html'
            },
        },
    },
});
