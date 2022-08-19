import * as fs from "fs";
import * as path from "path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { ServerOptions } from "https";

let https = {} as ServerOptions;
const keyFile = path.resolve(__dirname, "./.cert/key.pem");
const certFile = path.resolve(__dirname, "./.cert/cert.pem");
if (fs.existsSync(keyFile) && fs.existsSync(certFile)) {
  https = {
    key: fs.readFileSync(keyFile, "utf-8"),
    cert: fs.readFileSync(certFile, "utf-8"),
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
