<script lang="ts">
	import Canvas from '$lib/components/Canvas.svelte';
	import RowBackgroundImage from '$lib/components/containers/rows/RowBackgroundImage.svelte';
	import RowCarouselCover from '$lib/components/containers/rows/RowCarouselCover.svelte';
	import GridSimple from '$lib/components/grid/GridSimple.svelte';
	import Hr from '$lib/components/ui/Hr.svelte';
	import { getDataHomepage } from '$lib/functions/api';
	import { user_store } from '$lib/stores/user';
	import type { HomepageResponse } from '$lib/types/api';
	import { onMount } from 'svelte';

	const extractHomepageData = (
		data: HomepageResponse,
		key: keyof Omit<HomepageResponse, 'data'>
	) => {
		return data[key].map((it: string) => {
			return data.data[it];
		});
	};

	let state: 'loading' | 'error' | 'success' = $state('loading');
	let error: string | null = $state(null);
	let data: HomepageResponse | null = $state(null);

	onMount(() => {
		getDataHomepage()
			.then((res) => {
				data = {
					data: res.data,
					recent: [ ...res.recent, ...res.recent, ...res.recent, ...res.recent, ...res.recent, ...res.recent ],
					recent_added: [ ...res.recent_added, ...res.recent_added, ...res.recent_added, ...res.recent_added, ...res.recent_added, ...res.recent_added ]
				}
				state = 'success';
			})
			.catch((err) => {
				error = err.message;
				state = 'error';
			});
	});
</script>

<p>Login as : {$user_store?.username}</p>

<Canvas />

{#if state === 'loading'}
	<p>Loading...</p>
{:else if state === 'success' && data}
	<div class="flex flex-col gap-12">
		<RowCarouselCover games={extractHomepageData(data, 'recent_added')} />

		<Hr />
		<RowBackgroundImage title="Popular this month" games={extractHomepageData(data, 'recent')} />
		<GridSimple title="Recent games added" see_more_url="/" games={extractHomepageData(data, 'recent_added')} />
	</div>
	
{:else}
	<p>Error: {error}</p>
{/if}

{#await getDataHomepage()}
	<p>Loading...</p>
{:catch error}
	<p>Error: {error}</p>
{/await}

<a href="/gameboy">To gameboy</a>
