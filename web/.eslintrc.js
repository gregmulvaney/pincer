module.exports = {
  extends: ['@antfu'],
  overrides: [
    {
      files: ['*.js,*.vue'],
      processor: '@graphql-eslint/graphql',
    },
    {
      files: ['*.graphql'],
      parser: '@graphql-eslint/eslint-plugin',
      plugins: ['@graphql-eslint'],
      rules: {
        '@graphql-eslint/known-type-names': 'error',
      },
      parserOptions: {
        schema: './graphql/introspection.json',
      },
    },
  ],
}
