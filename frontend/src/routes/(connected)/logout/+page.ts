import { redirect } from "@sveltejs/kit"


export const load = async function() {
    redirect(302, '/login')
}