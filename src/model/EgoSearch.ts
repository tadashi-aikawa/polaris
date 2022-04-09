import type { Condition } from "./fetch-config";
import type { Message } from "~/model/search-messages";
import type { DateTime, Nullable } from "owlelia";

export type EgoSearchItem = {
  condition: Condition;
  messages: Message[];
  lastSearchDate?: DateTime;
};

export type EgoSearchLiquidValue = {
  item: EgoSearchItem;
  loading: boolean;
  error: Nullable<string>;
};
