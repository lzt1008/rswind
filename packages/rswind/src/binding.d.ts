/* auto-generated by NAPI-RS */
/* eslint-disable */
export class Application {
  generateWith(candidates: Array<[string, string]>): string
  generate(): string
  generateString(input: string, kind?: 'html' | 'ecma' | 'unknown'): string
  generateCandidate(input: Array<string>): string
}

export function createApp(options?: GeneratorOptions | undefined | null): Application

export interface GeneratorOptions {
  base?: string
  config?: string | false | AppConfig
  watch?: boolean
  parallel?: boolean
}

/* eslint-disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type StaticUtilityConfig =
  | {
      [k: string]: string;
    }
  | [unknown, unknown];
export type String = string;
export type UtilityGroup = "Transform" | "Filter" | "BackdropFilter";
export type OrderingKey =
  | "translate"
  | "translateAxis"
  | "scale"
  | "scaleAxis"
  | "rotate"
  | "rotateAxis"
  | "skew"
  | "skewAxis"
  | "transform"
  | "margin"
  | "marginAxis"
  | "marginSide"
  | "padding"
  | "paddingAxis"
  | "paddingSide"
  | "spaceAxis"
  | "rounded"
  | "roundedSide"
  | "roundedCorner"
  | "inset"
  | "insetAxis"
  | "insetSide"
  | "positionSide"
  | "borderSpacing"
  | "borderSpacingAxis"
  | "borderColor"
  | "borderColorAxis"
  | "borderColorSide"
  | "borderWidth"
  | "borderWidthAxis"
  | "borderWidthSide"
  | "size"
  | "sizeAxis"
  | "disorder"
  | "grouped"
  | "property";

export interface AppConfig {
  /**
   * The glob pattern to match input files
   */
  content?: string[];
  /**
   * How to handle `dark:` variant, can be `media` or `selector`
   */
  darkMode?: string;
  features?: Features;
  /**
   * User defined static utilities e.g. `flex`
   */
  staticUtilities?: {
    [k: string]: StaticUtilityConfig;
  };
  theme?: Theme;
  /**
   * User defined dynamic utilities, e.g. `bg-blue-500`
   */
  utilities?: UtilityBuilder[];
  [k: string]: unknown;
}
export interface Features {
  /**
   * Use a lexer to parse candidate, default to `true` if set to `false`, the parser will use regex to parse candidate
   */
  strict_mode: boolean;
  [k: string]: unknown;
}
/**
 * User define themes, will be merged with the default theme
 */
export interface Theme {
  [k: string]: MapOfString;
}
export interface MapOfString {
  [k: string]: string;
}
export interface UtilityBuilder {
  /**
   * The type validator for the utility, only used at `arbitrary values`
   *
   * e.g. `length-percentage` for `width`
   */
  type?: String | null;
  additionalCss?: MapOf_EitherStringOr_MapOfString | null;
  /**
   * The css handler for the utility, e.g. `background-color: $1`
   */
  css?: MapOfString | null;
  group?: UtilityGroup | null;
  /**
   * The key of the utility， e.g. `bg`
   */
  key: string;
  /**
   * The modifier for the utility, e.g. `bg-blue-500/50 <-`
   */
  modifier?: RawValueRepr | null;
  orderingKey?: OrderingKey | null;
  /**
   * Whether the utility supports fraction values, e.g. `w-1/2`
   */
  supportsFraction?: boolean;
  /**
   * Whether the utility supports negative values
   */
  supportsNegative?: boolean;
  /**
   * The theme key for the utility, will read from `theme` by this key later, e.g. `colors`
   */
  theme?: string | null;
  /**
   * The wrapper selector for the utility
   */
  wrapper?: string | null;
}
export interface MapOf_EitherStringOr_MapOfString {
  [k: string]: string | MapOfString;
}
/**
 * An unparsed value representation.
 *
 * This struct is used to store the raw value and modifier of a utility and will be parse into [`ValueRepr`].
 *
 * Used at: preset definitions, config deserialization.
 */
export interface RawValueRepr {
  type?: String | null;
  theme?: string | null;
  [k: string]: unknown;
}
