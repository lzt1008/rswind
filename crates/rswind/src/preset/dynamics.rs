use std::{
    mem,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use lazy_static::lazy_static;
use lightningcss::vendor_prefix::VendorPrefix;
use rswind_css::{rule::RuleList, Rule};
use rswind_css_macro::{css, rule_list};
use rswind_theme::{
    values::{FontFamily, FontSize},
    ThemeValue,
};
use smol_str::{format_smolstr, SmolStr};

use crate::{
    add_theme_utility,
    context::DesignSystem,
    ordering::OrderingKey,
    parsing::UtilityBuilder,
    process::{RawValueDef, RuleMatchingFn, UtilityGroup},
    types::{CssDataType, CssProperty},
};

pub fn load_dynamic_utilities(design: &mut DesignSystem) {
    let keyframes = design.get_theme("keyframes").unwrap_or_default();

    let mut rules = RuleAdder::new(design);

    rules
        .add("flex", |_, value| css!("flex": value))
        .support_fraction()
        .with_theme("flex")
        .with_validator(CssProperty::Flex(VendorPrefix::None));

    rules
        .add("shrink", |_, value| css!("flex-shrink": value))
        .with_theme("flexShrink")
        .with_validator(CssProperty::FlexShrink(VendorPrefix::None));

    rules
        .add("grow", |_, value| css!("flex-grow": value))
        .with_theme("flexGrow")
        .with_validator(CssProperty::FlexGrow(VendorPrefix::None));

    rules
        .add("basis", |_, value| css!("flex-basis": value))
        .with_theme(["flexBasis", "spacing"])
        .with_validator(CssProperty::FlexBasis(VendorPrefix::None));

    rules
        .add("origin", |_, value| css!("transform-origin": value))
        .with_theme("transformOrigin")
        .with_validator(CssProperty::TransformOrigin(VendorPrefix::None));

    rules
        .add("perspective", |_, value| css!("perspective": value))
        .with_validator(CssDataType::Length);

    rules
        .add("translate", |_, value| {
            css! {
                "--tw-translate-x": value.clone();
                "--tw-translate-y": value.clone();
                "--tw-translate-z": value;
                "translate": "translateX(var(--tw-translate-x)) translateY(var(--tw-translate-y))";
            }
        })
        .support_fraction()
        .support_negative()
        .with_theme(["translate", "spacing"])
        .with_validator(CssDataType::LengthPercentage)
        .with_additional_css(TRANSLATE_XYZ.clone());

    rules
        .add("translate-x", |_, value| {
            css! {
                "--tw-translate-x": value;
                "translate": "var(--tw-translate-x) var(--tw-translate-y)";
            }
        })
        .with_theme("translate")
        .with_validator(CssDataType::LengthPercentage)
        .support_fraction()
        .support_negative()
        .with_additional_css(TRANSLATE_XY.clone());

    rules
        .add("translate-y", |_, value| {
            css! {
                "--tw-translate-y": value;
                "translate": "var(--tw-translate-x) var(--tw-translate-y)";
            }
        })
        .with_theme("translate")
        .with_validator(CssDataType::LengthPercentage)
        .support_fraction()
        .support_negative()
        .with_additional_css(TRANSLATE_XY.clone());

    rules
        .add("translate-z", |_, value| {
            css! {
                "--tw-translate-z": value;
                "translate": "var(--tw-translate-x) var(--tw-translate-y) var(--tw-translate-z)";
            }
        })
        .with_theme("translate")
        .with_validator(CssDataType::LengthPercentage)
        .support_negative()
        // TODO: TRANSLATE_Z
        .with_additional_css(TRANSLATE_XYZ.clone());

    rules
        .add("scale", |_, value| {
            css! {
                "--tw-scale-x": value.clone();
                "--tw-scale-y": value.clone();
                "--tw-scale-z": value;
                "scale": "var(--tw-scale-x) var(--tw-scale-y)";
            }
        })
        .support_negative()
        .with_theme("scale")
        .with_validator(CssDataType::Percentage)
        .with_additional_css(SCALE_XYZ.clone());

    rules
        .add("scale-x", |_, value| {
            css! {
                "--tw-scale-x": value;
                "transform": "var(--tw-scale-x) var(--tw-scale-y)";
            }
        })
        .with_theme("scale")
        .with_validator(CssDataType::Percentage)
        .support_negative()
        .with_additional_css(SCALE_XY.clone());

    rules
        .add("scale-y", |_, value| {
            css! {
                "--tw-scale-y": value;
                "transform": "var(--tw-scale-x) var(--tw-scale-y)";
            }
        })
        .with_theme("scale")
        .with_validator(CssDataType::Percentage)
        .support_negative()
        .with_additional_css(SCALE_XY.clone());

    rules
        .add("scale-z", |_, value| {
            css! {
                "--tw-scale-z": value;
                "transform": "var(--tw-scale-x) var(--tw-scale-y) var(--tw-scale-z)";
            }
        })
        .with_theme("scale")
        .with_validator(CssDataType::Percentage)
        .support_negative()
        .with_additional_css(SCALE_XYZ.clone());

    rules
        .add("rotate", |_, value| css!("rotate": value))
        .with_theme("rotate")
        .support_negative()
        .with_validator(CssProperty::Rotate);

    rules
        .add("rotate-x", |_, value| css!("--tw-rotate-x": value))
        .with_theme("rotate")
        .support_negative()
        .with_validator(CssDataType::Angle)
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("rotate-y", |_, value| css!("--tw-rotate-y": value))
        .with_theme("rotate")
        .support_negative()
        .with_validator(CssDataType::Angle)
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("rotate-z", |_, value| css!("--tw-rotate-z": value))
        .with_theme("rotate")
        .support_negative()
        .with_validator(CssDataType::Angle)
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("skew", |_, value| {
            css! {
                "--tw-skew-x": value.clone();
                "--tw-skew-y": value;
            }
        })
        .with_theme("skew")
        .support_negative()
        .with_validator(CssDataType::Angle)
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("skew-x", |_, value| css!("--tw-skew-x": value))
        .with_theme("skew")
        .support_negative()
        .with_validator(CssDataType::Angle)
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("skew-y", |_, value| css!("--tw-skew-y": value))
        .with_theme("skew")
        .support_negative()
        .with_validator(CssDataType::Angle)
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("transform", |_, value| css!("transform": value))
        .with_validator(CssProperty::Transform(VendorPrefix::None))
        .with_group(UtilityGroup::Transform)
        .with_additional_css(TRANSFORM.clone());

    rules
        .add("line-clamp", |_, value| {
            css! {
                "display": "-webkit-box";
                "-webkit-line-clamp": value;
                "-webkit-box-orient": "vertical";
                "overflow": "hidden";
            }
        })
        .with_validator(CssDataType::Number)
        .with_theme("lineClamp");

    rules
        .add("border-spacing", |_, value| {
            css! {
                "--tw-border-spacing-x": value.clone();
                "--tw-border-spacing-y": value.clone();
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_ordering(OrderingKey::BorderSpacing)
        .with_theme(["borderSpacing", "spacing"])
        .with_validator(CssDataType::Length)
        .with_additional_css(BORDER_SPACING_XY.clone());

    rules
        .add("border-spacing-x", |_, value| {
            css! {
                "--tw-border-spacing-x": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_ordering(OrderingKey::BorderSpacingAxis)
        .with_theme(["borderSpacing", "spacing"])
        .with_validator(CssDataType::Length)
        .with_additional_css(BORDER_SPACING_XY.clone());

    rules
        .add("border-spacing-y", |_, value| {
            css! {
                "--tw-border-spacing-y": value;
                "border-spacing": "var(--tw-border-spacing-x) var(--tw-border-spacing-y)";
            }
        })
        .with_ordering(OrderingKey::BorderSpacingAxis)
        .with_theme(["borderSpacing", "spacing"])
        .with_validator(CssDataType::Length)
        .with_additional_css(BORDER_SPACING_XY.clone());

    rules
        .add("animate", |_, value| css!("animation": value))
        .with_theme("animation")
        .with_additional_css(move |value: SmolStr| {
            keyframes.get_rule_list(value.as_str()).cloned().map(|f| {
                Rule::new_with_rules(format_smolstr!("@keyframes {}", value), f).to_rule_list()
            })
        });

    rules
        .add("space-x", |_, value| {
            css! {
                "--tw-space-x-reverse": "0";
                "margin-right": format_smolstr!("calc({value} * var(--tw-space-x-reverse))");
                "margin-left":  format_smolstr!("calc({value} * calc(1 - var(--tw-space-x-reverse)))");
            }
        })
        .with_theme(["space", "spacing"])
        .with_selector("&:where(& > :not(:last-child))")
        .support_negative()
        .with_ordering(OrderingKey::SpaceAxis)
        .with_additional_css(SPACE_X_REVERSE.clone());

    rules
        .add("space-y", |_, value| {
            css! {
                "--tw-space-y-reverse": "0";
                "margin-top": format_smolstr!("calc({value} * calc(1 - var(--tw-space-y-reverse)))");
                "margin-bottom": format_smolstr!("calc({value} * var(--tw-space-y-reverse))");
            }
        })
        .with_theme(["space", "spacing"])
        .with_selector("&:where(& > :not(:last-child))")
        .support_negative()
        .with_ordering(OrderingKey::SpaceAxis)
        .with_additional_css(SPACE_Y_REVERSE.clone());

    rules.add("divide-x", |_, value| {
        css! {
            "--tw-divide-x-reverse": "0";
            "border-right-width": format_smolstr!("calc({value} * var(--tw-divide-x-reverse))");
            "border-left-width": format_smolstr!("calc({value} * calc(1 - var(--tw-divide-x-reverse)))");
        }
    })
    .with_theme(["divideWidth", "borderWidth"])
    .with_validator(CssProperty::BorderRightWidth)
    .with_ordering(OrderingKey::BorderWidthAxis)
    .with_additional_css(DIVIDE_X_REVERSE.clone());

    rules
        .add("divide-y", |_, value| {
            css! {
                "--tw-divide-y-reverse": "0";
                "border-top-width": format_smolstr!("calc({value} * calc(1 - var(--tw-divide-y-reverse)))");
                "border-bottom-width": format_smolstr!("calc({value} * var(--tw-divide-y-reverse))");
            }
        })
        .with_theme(["divideWidth", "borderWidth"])
        .with_validator(CssProperty::BorderTopWidth)
        .with_ordering(OrderingKey::BorderWidthAxis)
        .with_additional_css(DIVIDE_Y_REVERSE.clone());

    rules
        .add("divide", |m, v| css!("border-color": as_color(&v, m.modifier.as_deref())))
        .with_theme(["divideColor", "colors"])
        .with_validator(CssProperty::BorderColor)
        .with_opacity_modifier();

    rules
        .add(
            "border",
            |meta, value| css!("border-width": as_color(&value, meta.modifier.as_deref())),
        )
        .with_theme("colors")
        .with_validator(CssProperty::BorderColor)
        .with_opacity_modifier();

    rules
        .add("from", |_, value| {
            css! {
                "--tw-gradient-from": format_smolstr!("{value} var(--tw-gradient-from-position)");
                "--tw-gradient-stops": "var(--tw-gradient-via-stops, var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-to) var(--tw-gradient-to-position))";
            }
        })
        .with_theme(["gradientColorStops", "colors"])
        .with_ordering(OrderingKey::FromColor)
        .with_validator(CssProperty::Color)
        .with_opacity_modifier()
        .with_additional_css(GRADIENT_PROPERTIES.clone());

    rules
        .add("from", |_, value| css!("--tw-gradient-from-position": value))
        .with_theme("gradientColorStopPositions")
        .with_ordering(OrderingKey::FromPosition)
        .with_validator(CssDataType::LengthPercentage)
        .with_additional_css(GRADIENT_PROPERTIES.clone());

    rules.add("via", |meta, value| {
        css! {
            "--tw-gradient-via": as_color(&value, meta.modifier.as_deref());
            "--tw-gradient-via-stops": "var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-via) var(--tw-gradient-via-position), var(--tw-gradient-to) var(--tw-gradient-to-position)";
            "--tw-gradient-stops": "var(--tw-gradient-via-stops)";
        }
    })
    .with_theme(["gradientColorStops", "colors"])
    .with_ordering(OrderingKey::ViaColor)
    .with_validator(CssProperty::Color)
    .with_opacity_modifier()
    .with_additional_css(GRADIENT_PROPERTIES.clone());

    rules
        .add("via", |_, value| css!("--tw-gradient-via-position": value))
        .with_theme("gradientColorStopPositions")
        .with_ordering(OrderingKey::ViaPosition)
        .with_validator(CssDataType::LengthPercentage)
        .with_additional_css(GRADIENT_PROPERTIES.clone());

    rules.add("to", |meta, value| {
        css! {
            "--tw-gradient-to": as_color(&value, meta.modifier.as_deref());
            "--tw-gradient-stops": "var(--tw-gradient-via-stops, var(--tw-gradient-from) var(--tw-gradient-from-position), var(--tw-gradient-to) var(--tw-gradient-to-position))";
        }
    })
    .with_theme(["gradientColorStops", "colors"])
    .with_ordering(OrderingKey::ToColor)
    .with_additional_css(GRADIENT_PROPERTIES.clone());

    rules
        .add("to", |_, value| css!("--tw-gradient-to-position": value))
        .with_theme("gradientColorStopPositions")
        .with_validator(CssDataType::LengthPercentage);

    rules
        .add("fill", |meta, value| css!("fill": as_color(&value, meta.modifier.as_deref())))
        .with_theme(["fill", "colors"])
        .with_validator(CssProperty::Color);

    rules
        .add("stroke", |_, value| css!("stroke-width": value))
        .with_theme("strokeWidth")
        .with_validator(CssDataType::LengthPercentage);

    rules
        .add("stroke", |meta, value| css!("stroke": as_color(&value, meta.modifier.as_deref())))
        .with_validator(CssProperty::Color)
        .with_opacity_modifier()
        .with_theme(["stroke", "colors"]);

    rules
        .add(
            "bg",
            |meta, value| css!("background-color": as_color(&value, meta.modifier.as_deref())),
        )
        .with_theme(["backgroundColor", "colors"])
        .with_validator(CssProperty::Color)
        .with_opacity_modifier();

    rules
        .add("bg", |_, value| css!("background-position": value))
        .with_theme("backgroundPosition")
        .with_validator(CssProperty::BackgroundPosition);

    rules
        .add("bg", |_, value| css!("background-size": value))
        .with_theme("backgroundSize")
        .with_validator(CssProperty::BackgroundSize);

    rules
        .add("bg", |_, value| css!("background-image": value))
        .with_theme("backgroundImage")
        .with_validator(CssProperty::BackgroundImage);

    rules
        .add("text", |meta, value| css!("color": as_color(&value, meta.modifier.as_deref())))
        .with_theme(["textColor", "colors"])
        .with_validator(CssProperty::Color)
        .with_opacity_modifier();

    rules
        .add("text", move |meta, value| match meta.theme_value {
            ThemeValue::FontSize(font_size) => match font_size {
                FontSize::Plain(value) => css!("font-family": value.as_str()),
                FontSize::WithLineHeight((size, line_height)) => css! {
                    "font-size": size.as_str();
                    "line-height": line_height.as_str();
                },
                FontSize::WithConfig((value, config)) => {
                    config.apply(css!("font-size": value.as_str()))
                }
            },
            _ => css!("font-size": value),
        })
        .with_theme("fontSize")
        .with_validator(CssProperty::FontSize)
        .with_modifier(RawValueDef::new("lineHeight").with_validator(CssProperty::LineHeight));

    rules
        .add("font", |_, value| css!("font-weight": value))
        .with_theme("fontWeight")
        .with_validator(CssProperty::FontWeight);

    rules
        .add("font", |meta, value| match meta.theme_value {
            ThemeValue::FontFamily(font_family) => match font_family {
                FontFamily::Plain(value) => css!("font-family": value.as_str()),
                FontFamily::Multi(value) => css!("font-family": value.join(", ")),
                FontFamily::WithConfig((value, config)) => {
                    config.apply(css!("font-family": value.as_str()))
                }
            },
            _ => css!("font-family": value),
        })
        .with_theme("fontFamily")
        .with_validator(CssProperty::FontWeight);

    rules
        .add("text-ident", |_, value| css!("text-indent": value))
        .with_theme(["textIndent", "spacing"])
        .with_validator(CssProperty::TextIndent);

    rules
        .add("font-stretch", |_, value| css!("font-stretch": value))
        .with_validator(CssProperty::FontStretch);

    rules
        .add("placeholder", |meta, value| css!("color": as_color(&value, meta.modifier.as_deref())))
        .with_selector("&::placeholder")
        .with_theme(["placeholderColor", "colors"])
        .with_validator(CssProperty::Color)
        .with_opacity_modifier();

    rules
        .add("decoration", |meta, value| {
            css! {
                "text-decoration-color": as_color(&value, meta.modifier.as_deref());
            }
        })
        .with_theme(["textDecorationColor", "colors"])
        .with_validator(CssProperty::Color)
        .with_opacity_modifier();

    rules
        .add("decoration", |_, value| css!("text-decoration-thickness": value))
        .with_theme("textDecorationThickness")
        .with_validator(CssDataType::LengthPercentage);

    rules
        .add("shadow", |_, value| css!("box-shadow": value))
        .with_theme("boxShadow")
        .with_additional_css(BOX_SHADOW_PROPERTIES.clone())
        .with_validator(CssProperty::BoxShadow(VendorPrefix::None));

    rules
        .add("shadow", |_, value| {
            css! {
                "--tw-shadow-color": value;
                "--tw-shadow": "var(--tw-shadow-colored)";
            }
        })
        .with_theme(["boxShadowColor", "colors"])
        .with_additional_css(BOX_SHADOW_PROPERTIES.clone())
        .with_validator(CssProperty::Color);

    rules
        .add("blur", |_, value| css!("--tw-blur": format_smolstr!("blur({})", value)))
        .with_theme("blur")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-blur",
            |_, value| css!("--tw-backdrop-blur": format_smolstr!("blur({})", value)),
        )
        .with_theme(["backdropBlur", "blur"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add(
            "brightness",
            |_, value| css!("--tw-brightness": format_smolstr!("brightness({})", value)),
        )
        .with_theme("brightness")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-brightness",
            |_, value| css!("--tw-backdrop-brightness": format_smolstr!("brightness({})", value)),
        )
        .with_theme(["backdropBrightness", "brightness"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add("contrast", |_, value| css!("--tw-contrast": format_smolstr!("contrast({})", value)))
        .with_theme("contrast")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-contrast",
            |_, value| css!("--tw-backdrop-contrast": format_smolstr!("contrast({})", value)),
        )
        .with_theme(["backdropContrast", "contrast"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add(
            "grayscale",
            |_, value| css!("--tw-grayscale": format_smolstr!("grayscale({})", value)),
        )
        .with_theme("grayscale")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-grayscale",
            |_, value| css!("--tw-backdrop-grayscale": format_smolstr!("grayscale({})", value)),
        )
        .with_theme(["backdropGrayscale", "grayscale"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    //hue-rotate

    rules
        .add("invert", |_, value| css!("--tw-invert": format_smolstr!("invert({})", value)))
        .with_theme("invert")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-invert",
            |_, value| css!("--tw-backdrop-invert": format_smolstr!("invert({})", value)),
        )
        .with_theme(["backdropInvert", "invert"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add("invert", |_, value| css!("--tw-invert": format_smolstr!("invert({})", value)))
        .with_theme("invert")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-invert",
            |_, value| css!("--tw-backdrop-invert": format_smolstr!("invert({})", value)),
        )
        .with_theme(["backdropInvert", "invert"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add("saturate", |_, value| css!("--tw-saturate": format_smolstr!("saturate({})", value)))
        .with_theme("saturate")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-saturate",
            |_, value| css!("--tw-backdrop-saturate": format_smolstr!("saturate({})", value)),
        )
        .with_theme(["backdropSaturate", "saturate"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add("sepia", |_, value| css!("--tw-sepia": format_smolstr!("sepia({})", value)))
        .with_theme("sepia")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add(
            "backdrop-sepia",
            |_, value| css!("--tw-backdrop-sepia": format_smolstr!("sepia({})", value)),
        )
        .with_theme(["backdropSepia", "sepia"])
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules
        .add("drop-shadow", |_, value| css!("--tw-drop-shadow": value))
        .with_theme("dropShadow")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::Filter);

    rules
        .add("backdrop-opacity", |_, value| css!("--tw-backdrop-opacity": value))
        .with_theme("opacity")
        .with_validator(CssDataType::LengthPercentage)
        .with_group(UtilityGroup::BackdropFilter);

    rules.add("cursor", |_, value| css!("cursor": value)).with_validator(CssProperty::Cursor);

    rules
        .add("list", |_, value| css!("list-style-type": value))
        .with_validator(CssProperty::ListStyleType);

    rules
        .add("list-image", |_, value| css!("list-style-image": value))
        .with_validator(CssProperty::ListStyleImage);

    rules
        .add("columns", |_, value| css!("columns": value))
        // TODO: types
        .with_validator(CssDataType::Any);

    rules
        .add("auto-cols", |_, value| css!("grid-auto-columns": value))
        .with_theme("gridAutoColumns")
        .with_validator(CssProperty::GridAutoColumns);

    rules
        .add("auto-rows", |_, value| css!("grid-auto-rows": value))
        .with_theme("gridAutoRows")
        .with_validator(CssProperty::GridAutoRows);

    rules
        .add("gap", |_, value| css!("gap": value))
        .with_theme(["gap", "spacing"])
        .with_validator(CssProperty::Gap);

    rules
        .add("gap-x", |_, value| css!("column-gap": value))
        .with_theme(["gap", "spacing"])
        .with_validator(CssProperty::Gap);

    rules
        .add("gap-y", |_, value| css!("row-gap": value))
        .with_theme(["gap", "spacing"])
        .with_validator(CssProperty::Gap);

    rules
        .add(
            "accent",
            |meta, value| css!("accent-color": as_color(&value, meta.modifier.as_deref())),
        )
        .with_theme(["accentColor", "colors"])
        .with_validator(CssProperty::AccentColor);

    rules
        .add("caret", |meta, value| css!("caret-color": as_color(&value, meta.modifier.as_deref())))
        .with_theme(["caretColor", "colors"])
        .with_validator(CssProperty::AccentColor);

    rules
        .add("border", |_, value| {
            css! {
                "border-style": "var(--tw-border-style)";
                "border-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidth)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-x", |_, value| {
            css! {
                "border-left-style": "var(--tw-border-style)";
                "border-right-style": "var(--tw-border-style)";
                "border-left-width": value.clone();
                "border-right-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthAxis)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-y", |_, value| {
            css! {
                "border-top-style": "var(--tw-border-style)";
                "border-bottom-style": "var(--tw-border-style)";
                "border-top-width": value.clone();
                "border-bottom-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthAxis)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-s", |_, value| {
            css! {
                "border-inline-start-style": "var(--tw-border-style)";
                "border-inline-end-style": "var(--tw-border-style)";
                "border-inline-start-width": value.clone();
                "border-inline-end-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthSide)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-e", |_, value| {
            css! {
                "border-inline-start-style": "var(--tw-border-style)";
                "border-inline-end-style": "var(--tw-border-style)";
                "border-inline-start-width": value.clone();
                "border-inline-end-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthSide)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-t", |_, value| {
            css! {
                "border-top-style": "var(--tw-border-style)";
                "border-bottom-style": "var(--tw-border-style)";
                "border-top-width": value.clone();
                "border-bottom-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthSide)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-r", |_, value| {
            css! {
                "border-right-style": "var(--tw-border-style)";
                "border-left-style": "var(--tw-border-style)";
                "border-right-width": value.clone();
                "border-left-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthSide)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-b", |_, value| {
            css! {
                "border-top-style": "var(--tw-border-style)";
                "border-bottom-style": "var(--tw-border-style)";
                "border-top-width": value.clone();
                "border-bottom-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthSide)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("border-l", |_, value| {
            css! {
                "border-right-style": "var(--tw-border-style)";
                "border-left-style": "var(--tw-border-style)";
                "border-right-width": value.clone();
                "border-left-width": value;
            }
        })
        .with_theme("borderWidth")
        .with_validator(CssProperty::BorderWidth)
        .with_ordering(OrderingKey::BorderWidthSide)
        .with_additional_css(BORDER_STYLE.clone());

    rules
        .add("outline", |_, value| {
            css! {
                "outline-style": "var(--tw-outline-style)";
                "outline-width": value;
            }
        })
        .with_theme("outlineWidth")
        .with_validator(CssProperty::OutlineWidth);

    rules
        .add("outline", |_, value| css!("outline-color": value))
        .with_theme(["outlineColor", "colors"])
        .with_validator(CssProperty::OutlineWidth);

    rules
        .add(
            "ring",
            move |meta, value| css!("--tw-ring-color": as_color(&value, meta.modifier.as_deref())),
        )
        .with_theme(["ringColor", "colors"])
        .with_opacity_modifier()
        .with_additional_css(BOX_SHADOW_PROPERTIES.clone())
        .with_validator(CssProperty::BoxShadow(VendorPrefix::None));

    let default_ring_color = "currentColor";
    rules
        .add("ring", move |_, value| {
            css! {
                "--tw-ring-shadow": format!("var(--tw-ring-inset,) 0 0 0 calc(${value} + var(--tw-ring-offset-width)) var(--tw-ring-color, {})", default_ring_color);
            }
        })
        .with_theme(["ringColor", "colors"])

        .with_additional_css(BOX_SHADOW_PROPERTIES.clone())
        .with_validator(CssDataType::Length);

    rules.add("ring-offset", |_, value| {
        css! {
            "--tw-ring-offset-width": value;
            "--tw-ring-offset-shadow": "var(--tw-ring-inset,) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color)";
        }
    })
    .with_theme("ringOffsetWidth")
    .with_validator(CssDataType::Length);

    rules
        .add("ring-offset", |_, value| css!("--tw-ring-offset-color": value))
        // TODO: is this what we want?
        .with_theme(["ringOffsetColor", "ringColor", "colors"])
        .with_validator(CssProperty::Color)
        .with_opacity_modifier();

    {
        use lightningcss::properties::PropertyId::*;
        add_theme_utility!(design, {
            ["margin", "spacing"] => {
                // TODO: types, order
                "m" : Margin       => ["margin"]                      in OrderingKey::Margin, negative: true fraction: true
                "mx": MarginLeft   => ["margin-left", "margin-right"] in OrderingKey::MarginAxis
                "my": MarginTop    => ["margin-top", "margin-bottom"] in OrderingKey::MarginAxis
                "mt": MarginTop    => ["margin-top"]                  in OrderingKey::MarginSide
                "mr": MarginRight  => ["margin-right"]                in OrderingKey::MarginSide
                "mb": MarginBottom => ["margin-bottom"]               in OrderingKey::MarginSide
                "ml": MarginRight  => ["margin-left"]                 in OrderingKey::MarginSide
                "ms": MarginRight  => ["margin-inline-start"]         in OrderingKey::MarginSide
                "me": MarginRight  => ["margin-inline-end"]           in OrderingKey::MarginSide
            },
            ["padding", "spacing"] => {
                "p" : Padding    => ["padding"]                       in OrderingKey::Padding
                "px": PaddingTop => ["padding-left", "padding-right"] in OrderingKey::PaddingAxis
                "py": PaddingTop => ["padding-top", "padding-bottom"] in OrderingKey::PaddingAxis
                "pt": PaddingTop => ["padding-top"]                   in OrderingKey::PaddingSide
                "pr": PaddingTop => ["padding-right"]                 in OrderingKey::PaddingSide
                "pb": PaddingTop => ["padding-bottom"]                in OrderingKey::PaddingSide
                "pl": PaddingTop => ["padding-left"]                  in OrderingKey::PaddingSide
                "ps": PaddingTop => ["padding-inline-start"]          in OrderingKey::PaddingSide
                "pe": PaddingTop => ["padding-inline-end"]            in OrderingKey::PaddingSide
            },
            ["inset", "spacing"] => {
                "inset"   : Inset => ["top", "right", "bottom", "left"] in OrderingKey::Inset, negative: true fraction: true
                "inset-x" : Left  => ["left", "right"]                  in OrderingKey::InsetAxis, negative: true fraction: true
                "inset-y" : Top   => ["top", "bottom"]                  in OrderingKey::InsetAxis, negative: true fraction: true

                "top"   : Top => ["top"]    in OrderingKey::InsetSide, negative: true fraction: true
                "right" : Top => ["right"]  in OrderingKey::InsetSide, negative: true fraction: true
                "bottom": Top => ["bottom"] in OrderingKey::InsetSide, negative: true fraction: true
                "left"  : Top => ["left"]   in OrderingKey::InsetSide, negative: true fraction: true
            },
            ["width", "spacing"] => {
                "w": Width => ["width"] in OrderingKey::SizeAxis, fraction: true
            },
            ["maxWidth", "spacing"] => {
                "max-w": MaxWidth => ["max-width"] in OrderingKey::SizeAxis, fraction: true
            },
            ["minWidth", "spacing"] => {
                "min-w": MinWidth => ["min-width"] in OrderingKey::SizeAxis, fraction: true
            },
            ["height", "spacing"] => {
                "h": Width => ["height"] in OrderingKey::SizeAxis, fraction: true
            },
            ["maxHeight", "spacing"] => {
                "max-h": MaxWidth => ["max-height"] in OrderingKey::SizeAxis, fraction: true
            },
            ["minHeight", "spacing"] => {
                "min-h": MinWidth => ["min-height"] in OrderingKey::SizeAxis, fraction: true
            },
            ["size", "spacing"] => {
                "size": Width => ["width", "height"] in OrderingKey::Size, fraction: true
            },
            "borderRadius" => {
                "rounded"   : BorderRadius(VendorPrefix::None) => ["border-radius"] in OrderingKey::Rounded
                "rounded-s" : BorderRadius(VendorPrefix::None) => ["border-start-start-radius", "border-end-start-radius"] in OrderingKey::RoundedSide
                "rounded-e" : BorderRadius(VendorPrefix::None) => ["border-start-end-radius", "border-end-end-radius"] in OrderingKey::RoundedSide
                "rounded-t" : BorderRadius(VendorPrefix::None) => ["border-top-left-radius", "border-top-right-radius"] in OrderingKey::RoundedSide
                "rounded-r" : BorderRadius(VendorPrefix::None) => ["border-top-right-radius", "border-bottom-right-radius"] in OrderingKey::RoundedSide
                "rounded-b" : BorderRadius(VendorPrefix::None) => ["border-bottom-right-radius", "border-bottom-left-radius"] in OrderingKey::RoundedSide
                "rounded-l" : BorderRadius(VendorPrefix::None) => ["border-top-left-radius", "border-bottom-left-radius"] in OrderingKey::RoundedSide
                "rounded-ss": BorderRadius(VendorPrefix::None) => ["border-start-start-radius"] in OrderingKey::RoundedCorner
                "rounded-se": BorderRadius(VendorPrefix::None) => ["border-start-end-radius"] in OrderingKey::RoundedCorner
                "rounded-ee": BorderRadius(VendorPrefix::None) => ["border-end-end-radius"] in OrderingKey::RoundedCorner
                "rounded-es": BorderRadius(VendorPrefix::None) => ["border-end-start-radius"] in OrderingKey::RoundedCorner
                "rounded-tl": BorderRadius(VendorPrefix::None) => ["border-top-left-radius"] in OrderingKey::RoundedCorner
                "rounded-tr": BorderRadius(VendorPrefix::None) => ["border-top-right-radius"] in OrderingKey::RoundedCorner
                "rounded-br": BorderRadius(VendorPrefix::None) => ["border-bottom-right-radius"] in OrderingKey::RoundedCorner
                "rounded-bl": BorderRadius(VendorPrefix::None) => ["border-bottom-left-radius"] in OrderingKey::RoundedCorner
            },
            "lineHeight" => { "leading": LineHeight => ["line-height"] },
            ["borderColor", "colors"] => {
                // TODO: as_color
                "border"  : BorderColor => ["border-color"]                            in OrderingKey::BorderColor
                "border-x": BorderColor => ["border-right-color", "border-left-color"] in OrderingKey::BorderColorAxis
                "border-y": BorderColor => ["border-top-color", "border-bottom-color"] in OrderingKey::BorderColorAxis
                "border-s": BorderColor => ["border-inline-start-color"]               in OrderingKey::BorderColorSide
                "border-e": BorderColor => ["border-inline-end-color"]                 in OrderingKey::BorderColorSide
                "border-t": BorderColor => ["border-top-color"]                        in OrderingKey::BorderColorSide
                "border-r": BorderColor => ["border-right-color"]                      in OrderingKey::BorderColorSide
                "border-b": BorderColor => ["border-bottom-color"]                     in OrderingKey::BorderColorSide
                "border-l": BorderColor => ["border-left-color"]                       in OrderingKey::BorderColorSide
            },
            "opacity" => { "opacity": Opacity => ["opacity"] },
            ["scrollMargin", "spacing"] => {
                "scroll-m": ScrollMargin => ["scroll-margin"]
                "scroll-mx": ScrollMargin => ["scroll-margin-left", "scroll-margin-right"]
                "scroll-my": ScrollMargin => ["scroll-margin-top", "scroll-margin-bottom"]
                "scroll-ms": ScrollMargin => ["scroll-margin-inline-start"]
                "scroll-me": ScrollMargin => ["scroll-margin-inline-end"]
                "scroll-mt": ScrollMargin => ["scroll-margin-top"]
                "scroll-mr": ScrollMargin => ["scroll-margin-right"]
                "scroll-mb": ScrollMargin => ["scroll-margin-bottom"]
                "scroll-ml": ScrollMargin => ["scroll-margin-left"]
            },
            ["scrollPadding", "spacing"] => {
                "scroll-p": ScrollPadding => ["scroll-padding"]
                "scroll-px": ScrollPadding => ["scroll-padding-left", "scroll-padding-right"]
                "scroll-py": ScrollPadding => ["scroll-padding-top", "scroll-padding-bottom"]
                "scroll-ps": ScrollPadding => ["scroll-padding-inline-start"]
                "scroll-pe": ScrollPadding => ["scroll-padding-inline-end"]
                "scroll-pt": ScrollPadding => ["scroll-padding-top"]
                "scroll-pr": ScrollPadding => ["scroll-padding-right"]
                "scroll-pb": ScrollPadding => ["scroll-padding-bottom"]
                "scroll-pl": ScrollPadding => ["scroll-padding-left"]
            }
        });
    }
}

/// usage: property!(
/// "--tw-space-x-reverse", "<number>", "0";
/// "--tw-space-y-reverse", "<number>", "0";
/// );
///
/// or
/// property!(["x", "y"]; "--tw-translate", "<length-percentage>", "0")
macro_rules! property {
    ($($name:expr, $syn:literal $(, $initial:expr)? ;)*) => {
        rule_list! {
            $(
                concat!("@property ", $name) {
                    "syntax": concat!("\"", $syn, "\"");
                    "inherits": "false";
                    $("initial-value": $initial;)?
                }
            )*
        }
    };
    ([$($name:expr),*]; $prop:expr, $syn:literal, $initial:expr) => {
        rule_list! {
            $(
                concat!("@property ", $prop, "-", $name) {
                    "syntax": concat!("\"", $syn, "\"");
                    "inherits": "false";
                    "initial-value": $initial;
                }
            )*
        }
    };
}

lazy_static! {
    static ref TRANSLATE_XY: Arc<RuleList> =
        Arc::new(property!(["x", "y"]; "--tw-translate", "<length-percentage>", "0"));
    static ref TRANSLATE_XYZ: Arc<RuleList> =
        Arc::new(property!(["x", "y", "z"]; "--tw-translate", "<length-percentage>", "0"));
    static ref SCALE_XY: Arc<RuleList> =
        Arc::new(property!(["x", "y"]; "--tw-scale", "<number>", "1"));
    static ref SCALE_XYZ: Arc<RuleList> =
        Arc::new(property!(["x", "y", "z"]; "--tw-scale", "<number>", "1"));
    static ref ROTATE_XY: Arc<RuleList> =
        Arc::new(property!(["x", "y"]; "--tw-rotate", "<angle>", "0"));
    static ref ROTATE_XYZ: Arc<RuleList> =
        Arc::new(property!(["x", "y", "z"]; "--tw-rotate", "<angle>", "0"));
    static ref SKEW_XY: Arc<RuleList> =
        Arc::new(property!(["x", "y"]; "--tw-skew", "<angle>", "0"));
    static ref TRANSFORM: Arc<RuleList> =
        Arc::new(RuleList::from_list([ROTATE_XYZ.as_ref(), SKEW_XY.as_ref()]));
    static ref BORDER_SPACING_XY: Arc<RuleList> =
        Arc::new(property!(["x", "y"]; "--tw-border-spacing", "<length>", "0"));
    static ref SPACE_X_REVERSE: Arc<RuleList> =
        Arc::new(property!("--tw-space-x-reverse", "<number>", "0";));
    static ref SPACE_Y_REVERSE: Arc<RuleList> =
        Arc::new(property!("--tw-space-y-reverse", "<number>", "0";));
    static ref DIVIDE_X_REVERSE: Arc<RuleList> =
        Arc::new(property!("--tw-divide-x-reverse", "<number>", "0";));
    static ref DIVIDE_Y_REVERSE: Arc<RuleList> =
        Arc::new(property!("--tw-divide-y-reverse", "<number>", "0";));
    static ref GRADIENT_PROPERTIES: Arc<RuleList> = Arc::new(property!(
        "--tw-gradient-from", "<color>", "#0000";
        "--tw-gradient-to", "<color>", "#0000";
        "--tw-gradient-from", "<color>", "transparent";
        "--tw-gradient-via", "<color>", "transparent";
        "--tw-gradient-to", "<color>", "transparent";
        "--tw-gradient-stops", "*";
        "--tw-gradient-via-stops", "*";
        "--tw-gradient-from-position", "<length-percentage>", "0%";
        "--tw-gradient-via-position", "<length-percentage>", "50%";
        "--tw-gradient-to-position", "<length-percentage>", "100%";
    ));
    static ref BORDER_STYLE: Arc<RuleList> = Arc::new(property!(
        "--tw-border-style", "<custom-ident>", "solid";
    ));
    static ref BOX_SHADOW_PROPERTIES: Arc<RuleList> = Arc::new(property!(
        "--tw-shadow", "*", "0 0 #0000";
        "--tw-shadow-colored", "*", "0 0 #0000";
        "--tw-inset-shadow", "*", "0 0 #0000";
        "--tw-inset-shadow-colored", "*", "0 0 #0000";
        "--tw-ring-color", "*";
        "--tw-ring-shadow", "*", "0 0 #0000";
        "--tw-inset-ring-color", "*";
        "--tw-inset-ring-shadow", "*", "0 0 0 0 #0000";

        "--tw-ring-inset", "*";
        "--tw-ring-offset-width", "<length>", "0px";
        "--tw-ring-offset-color", "<color>", "#fff";
        "--tw-ring-offset-shadow", "*", "0 0 #0000";
    ));
}

pub fn as_color(value: &str, modifier: Option<&str>) -> SmolStr {
    modifier
        .and_then(|m| m.parse::<f32>().ok())
        .map(|n| format_smolstr!("color-mix(in srgb, {} {}%, transparent)", value, n * 100.0))
        .unwrap_or_else(|| value.into())
}

pub struct UtilityAdder<'i> {
    design: &'i mut DesignSystem,
    builder: UtilityBuilder,
}

impl<'i> Deref for UtilityAdder<'i> {
    type Target = UtilityBuilder;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for UtilityAdder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

impl<'i> UtilityAdder<'i> {
    pub fn new(
        design: &'i mut DesignSystem,
        key: &'i str,
        handler: impl RuleMatchingFn + 'static,
    ) -> Self {
        Self { design, builder: UtilityBuilder::new(key, handler) }
    }
}

/// Automatically adds the rule to the context when dropped.
/// This is useful for defining rules in a more declarative way.
impl<'i> Drop for UtilityAdder<'i> {
    fn drop(&mut self) {
        let builder = mem::take(&mut self.builder);
        let (key, utility) =
            builder.parse(&self.design.theme).unwrap_or_else(|e| panic!("{}", e.to_string()));

        self.design.extend(Some((key, utility)));
    }
}

struct RuleAdder<'a> {
    design: &'a mut DesignSystem,
}

impl<'a> RuleAdder<'a> {
    pub fn new(design: &'a mut DesignSystem) -> Self {
        Self { design }
    }

    pub fn add<'b>(
        &'b mut self,
        key: &'b str,
        handler: impl RuleMatchingFn + 'static,
    ) -> UtilityAdder<'b> {
        UtilityAdder::new(self.design, key, handler)
    }
}
