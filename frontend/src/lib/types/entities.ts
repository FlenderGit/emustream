import type { AuthTokens } from "./auth"


export type User = {
    username: string,
    // tokens: AuthTokens
}

export type Game = {
    id: string,
    title: string,
    slug: string,
    tags: string[],
    cover_url: string,
    developers: string[],
    created_at: string,
    release_date: string,
    releases: Array<{}>,
    metadata: boolean
}