export interface Response {
  user: User;
}

export interface User {
  real_name: String;
  display_name: String;
  avatar_hash: String;
  image_url: String;
}
