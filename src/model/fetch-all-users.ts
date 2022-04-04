import type { Nullable } from "owlelia";

export interface Response {
  user_by_id: UserMap;
}

export type UserMap = { [id: string]: User };

export interface User {
  id: string;
  name: string;
  real_name: Nullable<string>;
  images: Images;
}

export interface Images {
  image_48: string;
}
