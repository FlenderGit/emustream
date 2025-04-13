import { fetch_me } from '$lib/functions/api';
import { user_store } from '$lib/stores/user';
import { redirect } from '@sveltejs/kit';

export const load = async function () {
	try {
		const user = await fetch_me();
		console.log('user', user);
		if (user) {
            user_store.set(user);
			return { user };
		} else {
			redirect(302, '/login');
		}
	} catch (error) {
		redirect(302, '/login');
	}
};
