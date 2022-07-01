import * as fs from "fs";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

let https = {};
if (fs.existsSync("./.cert/key.pem") && fs.existsSync("./cert/cert.pem")) {
  https = {
    key: fs.readFileSync("./.cert/key.pem"),
    cert: fs.readFileSync("./.cert/cert.pem"),
  };
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    target: "es2020",
  },
  worker: {
    format: "es",
  },
  server: {
    https,
    fs: {
      allow: [".."],
    },
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp",
    },
  },
});
