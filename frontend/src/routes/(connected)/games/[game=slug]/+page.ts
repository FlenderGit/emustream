import { get_game_by_slug_and_console } from "$lib/functions/api";
import type { Game } from "$lib/types/entities";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {

    const game: Game | undefined = get_game_by_slug_and_console(params.game, params.console);

    console.log("Game", game);

    if (!game) {
        // console.error(`Game ${params.game} not found`);
        throw new Error(`Game ${params.game} not found`);
    }

    return {
        game
    };
};
