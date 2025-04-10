import { token_store } from '$lib/stores/tokens';
import { user_store } from '$lib/stores/user';
import type { HomepageResponse } from '$lib/types/api';
import type { AuthTokens } from '$lib/types/auth';
import type { User } from '$lib/types/entities';
import { get } from 'svelte/store';



const API_URL = import.meta.env.VITE_API_URL;

const fetch_api_restricted = function <T>(
	endpoint: string,
	init?: RequestInit,
	retry_count: number = 0
): Promise<T> {
	let tokens = get(token_store);
	if (!tokens) {
		return Promise.reject(new Error('User not logged in'));
	}
	let { access, refresh } = tokens;
	const headers: HeadersInit = new Headers();
	headers.append('Authorization', 'Bearer ' + access);
	headers.append('Content-Type', 'application/json');
	headers.append('Accept', 'application/json');
	return fetch(API_URL + endpoint, {
		headers,
		...init
	})
		.then(async (response) => {
			if (!response.ok) {
				if (response.status === 401) {
					if (retry_count > 1) {
						const new_tokens = await fetch_new_tokens(refresh);
						token_store.set(new_tokens);
						return fetch_api_restricted<T>(endpoint, init, retry_count - 1);
					} else {
						throw new Error('Failed to refresh tokens');
					}
				}
				throw new Error(`HTTP error! status: ${response.status}`);
			}
			return response.json() as Promise<T>;
		})
		.catch((error) => {
			console.error('Error fetching data:', error);
			throw error;
		});
};

const fetch_new_tokens = function (refresh_token: string): Promise<AuthTokens> {
	return fetch(API_URL + '/refresh', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			refresh: refresh_token
		})
	}).then((response) => {
		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}
		return response.json() as Promise<AuthTokens>;
	});
};

export const getDataHomepage = function (): Promise<HomepageResponse> {

    const data: HomepageResponse = {
        "recent": [
            "67f7c9ae2bf3047e6b740fd0"
        ],
        "recent_added": [
            "67f7c9ae2bf3047e6b740fd0",
            "67f7c9ae2bf3047e6b740fcf"
        ],
        "data": {
            "67f7c9ae2bf3047e6b740fcf": {
                "id": "67f7c9ae2bf3047e6b740fcf",
                "title": "CLANNAD",
                "tags": [
                    "VN",
                    "Drama"
                ],
                "cover_url": "https://t.vndb.org/cv.t/95/75895.jpg",
                "developers": [
                    "Key"
                ],
                "release_date": "1970-01-01T00:00:01.674Z",
                "created_at": "2025-04-10T13:37:50.375788744Z",
                "releases": [
                    {
                        "title": "Clannad - First press edition",
                        "platforms": [
                            "PS4"
                        ],
                        "languages": [
                            "JP",
                            "EN"
                        ],
                        "region": null,
                        "release_date": "1970-01-01T00:00:01.674Z",
                        "created_at": "2025-04-10T13:37:50.375783043Z",
                        "path": "CLANNAD"
                    },
                    {
                        "title": "Clannad - First press edition",
                        "platforms": [
                            "WIN"
                        ],
                        "languages": [
                            "JP"
                        ],
                        "region": null,
                        "release_date": "1970-01-01T00:00:01.674Z",
                        "created_at": "2025-04-10T13:37:50.375786444Z",
                        "path": "CLANNAD-fp"
                    }
                ],
                "metadata": true
            },
            "67f7c9ae2bf3047e6b740fd0": {
                "id": "67f7c9ae2bf3047e6b740fd0",
                "title": "Pokemon red",
                "tags": [
                    "RPG",
                    "Pokemon"
                ],
                cover_url: "https://www.nintendo.com/eu/media/images/10_share_images/games_15/game_boy_4/H2x1_GB_PokemonRed_frFR_image1600w.jpg",
                "developers": [
                    "Game Freak"
                ],
                "release_date": "1970-01-01T00:00:01.674Z",
                "created_at": "2025-04-10T13:37:50.385043675Z",
                "releases": [
                    {
                        "title": "Pokemon version rouge",
                        "platforms": [
                            "Gameboy"
                        ],
                        "languages": [
                            "EN"
                        ],
                        "region": null,
                        "release_date": "1970-01-01T00:00:01.674Z",
                        "created_at": "2025-04-10T13:37:50.385041275Z",
                        "path": "pokemon-red"
                    }
                ],
                "metadata": true
            }
        }
    };

    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(data);
        }, 1000);
    });

	return fetch_api_restricted<HomepageResponse>('/api/games/homepage');
};

export const fetch_me = function(): Promise<User> {

    const data: User = {
        username: "test"
    };

    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(data);
        }, 1000);
    });

    return fetch_api_restricted('/api/me')
}
