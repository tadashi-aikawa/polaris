import { writable } from "svelte/store";
import type { EmojiMap } from "~/model/fetch-emoji-list";
import type { UserMap } from "~/model/fetch-all-users";
import type { UserGroupMap } from "~/model/fetch-all-usergroups";
import type { DateTime } from "owlelia";

export const emojiMap = writable<EmojiMap>({});
export const userMap = writable<UserMap>({});
export const userGroupMap = writable<UserGroupMap>({});
export const readById = writable<{ [messageId: string]: DateTime }>({});
