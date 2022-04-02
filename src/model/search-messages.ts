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
  | {
      type: "rich_text_section";
      elements: Element[];
    }
  | {
      type: "rich_text_quote";
      elements: Element[];
    }
  | {
      type: "rich_text_list";
      style: "bullet";
      indent: number;
      elements: Element[];
    }
  | {
      type: "rich_text_preformatted";
      border: number;
      elements: Element[];
    }
  | {
      type: "emoji";
      name: string;
    }
  | {
      type: "text";
      text: string;
      style: Nullable<Style>;
    }
  | {
      type: "link";
      url: string;
      text: Nullable<string>;
    }
  | {
      type: "user";
      user_id: string;
    }
  | {
      type: "broadcast";
      range: "channel" | "here";
    }
  | {
      type: "channel";
      channel_id: string;
      style: Nullable<Style>;
    };

export interface Style {
  code: Nullable<boolean>;
  bold: Nullable<boolean>;
}
