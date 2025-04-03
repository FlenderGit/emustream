import type { PageLoad } from "../home/$types";


export const load: PageLoad = async ({ params }) => {
    return {
        console_slug: params.console,
    }
}
