import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {

    type Console = {
        name: string;
        slug: string;
        image: string;
    };

	const promise: Promise<Console> = new Promise((resolve) => {
		setTimeout(() => {
			resolve({
                name: 'PlayStation 5',
                slug: 'ps5',
                image: 'https://www.playstation.com/etc.clientlibs/ps5/clientlibs/ps5/resources/images/ps5-console.png',
            });
		}, 2500);
	});

	return {
		console_slug: params.console,
        console: promise,
	};
};
