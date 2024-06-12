import { defineConfig } from "vite";
import nunjucks from "nunjucks";
import { minify } from "html-minifier-terser";

function nunjucksPlugin() {
    return {
        name: "vite-plugin-nunjucks",
        transformIndexHtml(html) {
            const env = nunjucks.configure("./src/templates", { autoescape: true });
            return minify(env.renderString(html), {
                removeComments: true,
                collapseWhitespace: true,
                collapseBooleanAttributes: true,
                removeAttributeQuotes: true,
                removeEmptyAttributes: true,
            });
        },
    };
}

export default defineConfig({
    plugins: [
        nunjucksPlugin(),
    ],
});
