import type { Nullable } from "owlelia";

export interface Response {
  messages: Message[];
}

export interface Message {
  id: string;
  user_id: string;
  user_name: string;
  channel_name: string;
  text: string;
  permalink: string;
  created_at: string;
  blocks: Nullable<Block[]>;
}

export interface Block {
  block_type: string;
  block_id: string;
  elements: Element[];
}

export type Element =
  | RichTextSectionElement
  | RichTextQuoteElement
  | RichTextListElement
  | RichTextPreformattedElement
  | EmojiElement
  | TextElement
  | LinkElement
  | UserElement
  | UserGroupElement
  | BroadcastElement
  | ChannelElement;

export type RichTextSectionElement = {
  type: "rich_text_section";
  elements: Element[];
};

export type RichTextQuoteElement = {
  type: "rich_text_quote";
  elements: Element[];
};

export type RichTextListElement = {
  type: "rich_text_list";
  style: "bullet";
  indent: number;
  elements: Element[];
};

export type RichTextPreformattedElement = {
  type: "rich_text_preformatted";
  border: number;
  elements: Element[];
};

export type EmojiElement = {
  type: "emoji";
  name: string;
};

export type TextElement = {
  type: "text";
  text: string;
  style: Nullable<Style>;
};

export type LinkElement = {
  type: "link";
  url: string;
  text: Nullable<string>;
};

export type UserElement = {
  type: "user";
  user_id: string;
};

export type UserGroupElement = {
  type: "usergroup";
  usergroup_id: string;
};

export type BroadcastElement = {
  type: "broadcast";
  range: "channel" | "here";
  style: Nullable<Style>;
};

export type ChannelElement = {
  type: "channel";
  channel_id: string;
  style: Nullable<Style>;
};

export interface Style {
  code: Nullable<boolean>;
  bold: Nullable<boolean>;
}
