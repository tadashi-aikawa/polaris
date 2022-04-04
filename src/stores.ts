import { writable } from "svelte/store";
import type { EmojiMap } from "~/model/fetch-emoji-list";
import type { UserMap } from "~/model/fetch-all-users";

export const emojiMap = writable<EmojiMap>({});
export const userMap = writable<UserMap>({});
