import type { PageLoad } from "./$types"

export const load: PageLoad = async ({ params }) => {
    return {
        game_slug: params.game,
    }
}