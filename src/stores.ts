import { writable } from "svelte/store";
import type { EmojiMap } from "~/model/fetch-emoji-list";
import type { UserMap } from "~/model/fetch-all-users";
import type { UserGroupMap } from "~/model/fetch-all-usergroups";
import type { DateTime } from "owlelia";
import avatar from "~/assets/stranger.png";

export const emojiMap = writable<EmojiMap>({});
export const userGroupMap = writable<UserGroupMap>({});
export const readById = writable<{ [messageId: string]: DateTime }>({});

export const userMap = (function () {
  const { set, subscribe } = writable<UserMap>({});
  let state: UserMap;
  subscribe((v) => {
    state = v;
  });
  return {
    subscribe,
    set,
    getUserById: (id: string) =>
      state[id] ?? {
        id,
        name: "Unknown user",
        real_name: "Unknown user",
        images: {
          image_32: avatar,
          image_48: avatar,
          image_512: avatar,
        },
      },
  };
})();
