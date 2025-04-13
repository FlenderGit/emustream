<script>

	import { page } from "$app/state";

</script>
<p>{ page.status } - { JSON.stringify(page.error) }</p>