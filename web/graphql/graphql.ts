/* eslint-disable */
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type Model = {
  __typename?: 'Model';
  adjustedSize: Scalars['String'];
  host: Scalars['String'];
  id: Scalars['Int'];
  name: Scalars['String'];
  rawSize: Scalars['String'];
  unit: Scalars['String'];
  url: Scalars['String'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  addDownload: Scalars['Boolean'];
};


export type MutationRootAddDownloadArgs = {
  url: Scalars['String'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  downloads: Array<Model>;
  greet: Scalars['String'];
  pauseDownloads: Scalars['String'];
  startDownloads: Scalars['String'];
  stopDownloads: Scalars['String'];
};


export type QueryRootGreetArgs = {
  name: Scalars['String'];
};

export type SubscriptionRoot = {
  __typename?: 'SubscriptionRoot';
  getDownloads: Scalars['Boolean'];
};
