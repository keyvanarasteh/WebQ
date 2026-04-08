// vite.config.ts
import { defineConfig } from "file:///home/drvoid/ISU/WebQ/node_modules/vite/dist/node/index.js";
import { sveltekit } from "file:///home/drvoid/ISU/WebQ/node_modules/@sveltejs/kit/src/exports/vite/index.js";
import tailwindcss from "file:///home/drvoid/ISU/WebQ/node_modules/@tailwindcss/vite/dist/index.mjs";
import { paraglide } from "file:///home/drvoid/ISU/WebQ/node_modules/@inlang/paraglide-sveltekit/dist/vite/index.js";
var host = process.env.TAURI_DEV_HOST;
var vite_config_default = defineConfig(async () => ({
  plugins: [
    tailwindcss(),
    paraglide({ project: "./project.inlang", outdir: "./src/lib/paraglide" }),
    sveltekit()
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? {
      protocol: "ws",
      host,
      port: 1421
    } : void 0,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9kcnZvaWQvSVNVL1dlYlFcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIi9ob21lL2Rydm9pZC9JU1UvV2ViUS92aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vaG9tZS9kcnZvaWQvSVNVL1dlYlEvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xuaW1wb3J0IHsgc3ZlbHRla2l0IH0gZnJvbSBcIkBzdmVsdGVqcy9raXQvdml0ZVwiO1xuaW1wb3J0IHRhaWx3aW5kY3NzIGZyb20gXCJAdGFpbHdpbmRjc3Mvdml0ZVwiO1xuaW1wb3J0IHsgcGFyYWdsaWRlIH0gZnJvbSBcIkBpbmxhbmcvcGFyYWdsaWRlLXN2ZWx0ZWtpdC92aXRlXCI7XG5cbmNvbnN0IGhvc3QgPSBwcm9jZXNzLmVudi5UQVVSSV9ERVZfSE9TVDtcblxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XG4gIHBsdWdpbnM6IFtcbiAgICB0YWlsd2luZGNzcygpLFxuICAgIHBhcmFnbGlkZSh7IHByb2plY3Q6ICcuL3Byb2plY3QuaW5sYW5nJywgb3V0ZGlyOiAnLi9zcmMvbGliL3BhcmFnbGlkZScgfSksXG4gICAgc3ZlbHRla2l0KClcbiAgXSxcblxuICBjbGVhclNjcmVlbjogZmFsc2UsXG4gIHNlcnZlcjoge1xuICAgIHBvcnQ6IDE0MjAsXG4gICAgc3RyaWN0UG9ydDogdHJ1ZSxcbiAgICBob3N0OiBob3N0IHx8IGZhbHNlLFxuICAgIGhtcjogaG9zdFxuICAgICAgPyB7XG4gICAgICAgICAgcHJvdG9jb2w6IFwid3NcIixcbiAgICAgICAgICBob3N0LFxuICAgICAgICAgIHBvcnQ6IDE0MjEsXG4gICAgICAgIH1cbiAgICAgIDogdW5kZWZpbmVkLFxuICAgIHdhdGNoOiB7XG4gICAgICBpZ25vcmVkOiBbXCIqKi9zcmMtdGF1cmkvKipcIl0sXG4gICAgfSxcbiAgfSxcbn0pKTtcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBaVAsU0FBUyxvQkFBb0I7QUFDOVEsU0FBUyxpQkFBaUI7QUFDMUIsT0FBTyxpQkFBaUI7QUFDeEIsU0FBUyxpQkFBaUI7QUFFMUIsSUFBTSxPQUFPLFFBQVEsSUFBSTtBQUV6QixJQUFPLHNCQUFRLGFBQWEsYUFBYTtBQUFBLEVBQ3ZDLFNBQVM7QUFBQSxJQUNQLFlBQVk7QUFBQSxJQUNaLFVBQVUsRUFBRSxTQUFTLG9CQUFvQixRQUFRLHNCQUFzQixDQUFDO0FBQUEsSUFDeEUsVUFBVTtBQUFBLEVBQ1o7QUFBQSxFQUVBLGFBQWE7QUFBQSxFQUNiLFFBQVE7QUFBQSxJQUNOLE1BQU07QUFBQSxJQUNOLFlBQVk7QUFBQSxJQUNaLE1BQU0sUUFBUTtBQUFBLElBQ2QsS0FBSyxPQUNEO0FBQUEsTUFDRSxVQUFVO0FBQUEsTUFDVjtBQUFBLE1BQ0EsTUFBTTtBQUFBLElBQ1IsSUFDQTtBQUFBLElBQ0osT0FBTztBQUFBLE1BQ0wsU0FBUyxDQUFDLGlCQUFpQjtBQUFBLElBQzdCO0FBQUEsRUFDRjtBQUNGLEVBQUU7IiwKICAibmFtZXMiOiBbXQp9Cg==
