import { writable } from 'svelte/store'
import type { User } from './bindings/User'
import type { Link } from './bindings/Link'

export const user = writable<User>()

export const links = writable<Link[]>([])

export const link_id = writable<number>(0)
