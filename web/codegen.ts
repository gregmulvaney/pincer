import {CodegenConfig} from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: '../graphql/schema.gql',
  generates: {
    './graphql/': {
      preset: 'client',
      plugins: []
    },
    './graphql/introspection.json': {
      plugins: ['introspection'],
      config: {
        minify: true
      },
    },
  }
}

export default config