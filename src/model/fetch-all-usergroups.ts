export interface Response {
  usergroup_by_id: UserGroupMap;
}

export type UserGroupMap = { [id: string]: UserGroup };

export interface UserGroup {
  id: string;
  name: string;
}
