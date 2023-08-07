import { writable } from 'svelte/store'
import type { Profile } from './bindings/Profile'
import type { Link } from './bindings/Link'

export const user = writable<Profile>()

export const links = writable<Link[]>([])

export const link_id = writable<number>(0)
