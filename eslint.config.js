import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';
import globals from 'globals';

export default [
  js.configs.recommended,
  {
    files: ['**/*.ts', '**/*.svelte'],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.es2022,
      },
      ecmaVersion: 2022,
      sourceType: 'module',
    },
  },
  {
    files: ['**/*.ts'],
    languageOptions: {
      parser: tsParser,
    },
    plugins: {
      '@typescript-eslint': ts,
    },
    rules: {
      ...ts.configs.recommended.rules,
    },
  },
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tsParser,
      },
    },
    plugins: {
      svelte,
    },
    rules: {
      ...svelte.configs.recommended.rules,
      // Svelte 5 runes use special syntax that ESLint doesn't understand
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': 'off',
      // Svelte reactivity pattern: config = config
      'no-self-assign': 'off',
    },
  },
  {
    ignores: [
      'node_modules/**',
      '.svelte-kit/**',
      'build/**',
      'dist/**',
      'src-tauri/target/**',
      '*.config.js',
      '*.config.ts',
    ],
  },
];
