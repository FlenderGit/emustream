import type { AuthTokens } from "./auth"


export type User = {
    username: string,
    tokens: AuthTokens
}