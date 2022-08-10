module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:import/typescript",
    "plugin:prettier/recommended",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: "latest",
    sourceType: "module",
    tsconfigRootDir: ".",
    project: ["./tsconfig.json", "./tsconfig.eslint.json"],
  },
  plugins: ["@typescript-eslint", "prettier", "import", "prefer-arrow"],
  rules: {
    quotes: ["warn", "double", { allowTemplateLiterals: true }],
    "import/extensions": ["error", "never"],
    "prettier/prettier": ["warn", { endOfLine: "auto" }],
    "@typescript-eslint/no-shadow": ["error"],
    "no-restricted-imports": [
      "error",
      {
        patterns: [
          {
            group: ["**/api/*"],
            message: "Reexport from another module instead.",
          },
          {
            group: ["**/entities/*"],
            message: "Reexport from parent module instead.",
          },
        ],
      },
    ],
    "import/order": [
      "error",
      {
        groups: [
          ["builtin", "external"],
          "internal",
          "parent",
          ["index", "sibling"],
        ],
        pathGroups: [
          {
            pattern: "solid-js",
            group: "external",
            position: "before",
          },
          // {
          //   pattern: "@mui/**",
          //   group: "external",
          //   position: "after",
          // },
          // {
          //   pattern: "@paton/**",
          //   group: "internal",
          //   position: "before",
          // },
        ],
        pathGroupsExcludedImportTypes: [],
        "newlines-between": "always",
        alphabetize: { order: "asc" },
        warnOnUnassignedImports: true,
      },
    ],
    "import/prefer-default-export": "off",
    "import/no-default-export": "error",
    "prefer-arrow/prefer-arrow-functions": "error",
    "no-console": "warn",
  },
  overrides: [
    {
      files: ["**/*.test.{ts,tsx}"],
      env: {
        jest: true,
      },
    },
  ],
};
