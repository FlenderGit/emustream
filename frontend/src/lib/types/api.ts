import type { Game } from "./entities";

export type HomepageResponse = {
	data: Record<string, Game>;
	recent: Array<string>;
    recent_added: Array<string>;
};