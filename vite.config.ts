// SPDX-FileCopyrightText: 2022 - 2023 Kamila Borowska <kamila@borowska.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  test: {
    environment: "jsdom",
    deps: {
      optimizer: {
        web: {
          exclude: ["solid-js"],
        },
      },
    },
  },
  plugins: [solidPlugin()],
  build: {
    manifest: true,
    rollupOptions: {
      input: "js/index.ts",
    },
  },
});
