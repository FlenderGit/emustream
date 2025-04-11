<script lang="ts">
	import RowBackgroundImage from '$lib/components/containers/rows/RowBackgroundImage.svelte';
	import GridSimple from '$lib/components/grid/GridSimple.svelte';
	import { getDataHomepage } from '$lib/functions/api';
	import { user_store } from '$lib/stores/user';
	import type { HomepageResponse } from '$lib/types/api';

	const extractHomepageData = (data: HomepageResponse, key: keyof Omit<HomepageResponse, "data">) => {
		return data[key].map((it: string) => {
			return data.data[it];
		});
	};
</script>

<p>Login as : {$user_store?.username}</p>

{#await getDataHomepage()}
	<p>Loading...</p>
{:then data}
	<RowBackgroundImage games={extractHomepageData(data, 'recent')} />
	<GridSimple games={extractHomepageData(data, 'recent_added')} />
{:catch error}
	<p>Error: {error}</p>
{/await}

<a href="/gameboy">To gameboy</a>
