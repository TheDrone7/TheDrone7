/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution');

module.exports = {
  root: true,
  extends: [
    'plugin:vue/vue3-essential',
    'eslint:recommended',
    '@vue/eslint-config-typescript',
    '@vue/eslint-config-prettier/skip-formatting'
  ],
  overrides: [
    {
      files: [
        '**/__tests__/*.{cy,spec}.{js,ts,jsx,tsx}',
        'cypress/e2e/**/*.{cy,spec}.{js,ts,jsx,tsx}',
        'cypress/support/**/*.{js,ts,jsx,tsx}'
      ],
      extends: ['plugin:cypress/recommended']
    }
  ],
  rules: {
    'linebreak-style': ['error', 'unix'],
    'no-duplicate-imports': 'error',
    'no-async-promise-executor': 'off',
    'no-use-before-define': 'error',
    curly: ['error', 'multi'],
    'dot-notation': 'error',
    eqeqeq: ['error', 'always'],
    'prefer-arrow-callback': 'error',
    'prefer-const': 'error',
    'multiline-ternary': ['error', 'always-multiline'],
    'no-extra-parens': 'off',
    'no-trailing-spaces': ['error', { skipBlankLines: true }],
    'prettier/prettier': 2,
    'no-unused-vars': 'off',
    'max-len': ['error', 120, { ignoreTemplateLiterals: true, ignoreUrls: true }],
    'no-case-declarations': 'off'
  },
  ignorePatterns: ['**/icons/*.vue'],
  parserOptions: {
    ecmaVersion: 'latest'
  }
};
