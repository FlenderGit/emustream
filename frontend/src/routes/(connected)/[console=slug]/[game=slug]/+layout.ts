import type { PageLoad } from "../../home/$types"

export const load: PageLoad = async ({ params }) => {
    return {
        game_slug: params.game,
    }
}