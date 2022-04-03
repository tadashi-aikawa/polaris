import { writable } from "svelte/store";
import type { EmojiMap } from "~/model/fetch-emoji-list";

export const emojiMap = writable<EmojiMap>({});
