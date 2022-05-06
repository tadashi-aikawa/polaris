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
  attachments: Nullable<Attachment[]>;
  blocks: Nullable<Block[]>;
}

// なんとか共用体にしたい
export interface Attachment {
  from_url: Nullable<string>;
  author_name: Nullable<string>;
  author_subname: Nullable<string>;
  author_icon: Nullable<string>;
  channel_name: string;
  message_blocks: Nullable<MessageBlockElement[]>;
  // service
  image_url: Nullable<string>;
  image_width: Nullable<number>;
  image_height: Nullable<number>;
  text: Nullable<string>;
  footer: Nullable<string>;
  footer_icon: Nullable<string>;
}

export interface MessageBlockElement {
  team: string;
  channel: string;
  ts: string;
  message: {
    blocks: Block[];
  };
}

export interface Block {
  block_type: string;
  block_id: string;
  elements: Nullable<Element[]>;
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
  style: "bullet" | "ordered";
  indent: number;
  elements: Element[];
};

export type RichTextPreformattedElement = {
  type: "rich_text_preformatted";
  border?: number;
  elements: Element[];
};

export type EmojiElement = {
  type: "emoji";
  name: string;
  style: Nullable<Style>;
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
  strike: Nullable<boolean>;
}
