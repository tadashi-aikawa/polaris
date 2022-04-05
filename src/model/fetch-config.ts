import type { Nullable } from "owlelia";

export interface Response {
  interval_sec: number;
  conditions: Condition[];
  since_day_ago: Nullable<number>;
}

export interface Condition {
  title: Nullable<string>;
  color: Nullable<string>;
  query: string;
  interval_sec: Nullable<number>;
  should_notify: Nullable<boolean>;
  include_me: Nullable<boolean>;
}
