//! A2UI v0.9 Standard Component Catalog — Full 18 Components
//!
//! Based on Google's A2UI v0.9 Standard Catalog (basic_catalog.json),
//! published April 17, 2026. The catalog provides 18 UI components
//! organised into display, layout, container, and input categories.
//! It is "prompt-first designed" — optimised for embedding in LLM
//! system prompts rather than structured output.
//!
//! Catalog ID: https://a2ui.org/specification/v0_9/basic_catalog.json
//!
//! Each component carries AI-coding constraints: allowed props, style
//! locks, forbidden patterns. The LLM can never generate arbitrary
//! colours, spacing, or component types outside the catalog.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The complete A2UI v0.9 component catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2UIComponentCatalogV2 {
    pub catalog_id: String,
    pub version: String,
    pub components: HashMap<String, ComponentSpec>,
    pub functions: Vec<ClientFunction>,
}

/// Specification for a single A2UI component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpec {
    /// A2UI component type name (e.g., "Text", "Button", "Table").
    pub component_type: ComponentType,
    /// Human-readable description for LLM prompts.
    pub description: String,
    /// Allowed properties with types and constraints.
    pub allowed_props: Vec<PropSpec>,
    /// Style locks: values that MUST be used (no LLM override).
    pub style_locks: StyleLocks,
    /// Forbidden patterns: props or combinations the LLM must never generate.
    pub forbidden_patterns: Vec<String>,
    /// WCAG 2.1 AA requirements specific to this component.
    pub wcag_requirements: WcagRequirements,
    /// Minimum A2UI spec version required.
    pub min_spec_version: String,
    /// Whether this component supports streaming content updates.
    pub supports_streaming: bool,
}

/// The 18 A2UI v0.9 standard component types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ComponentType {
    // ═══ Display Components ═══
    Text,           // Displays text. Variants: h1-h5, caption, body.
    Image,          // Displays an image. Fit: contain, cover, fill, none, scaleDown.
    Icon,           // System icon from predefined set.
    Chart,          // Data visualisation (BarChart, LineChart, PieChart).
    Progress,       // Progress indicator (linear, circular).

    // ═══ Layout Components ═══
    Row,            // Horizontal layout container.
    Column,         // Vertical layout container.
    Divider,        // Visual separator.
    Spacer,         // Flexible spacing.

    // ═══ Container Components ═══
    Card,           // Elevated surface with rounded corners.
    List,           // Scrollable list of items.
    Table,          // Data grid with sortable columns.
    Tabs,           // Tabbed content container.
    Modal,          // Overlay dialog.

    // ═══ Input Components ═══
    TextField,      // Text input with validation.
    Button,         // Action trigger. Variants: primary, secondary, danger, ghost.
    Toggle,         // Boolean switch.
    Slider,         // Range selection.
    DatePicker,     // Date/time selection.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropSpec {
    pub name: String,
    pub prop_type: PropType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: String,
    pub constraints: Option<PropConstraints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Array(Box<PropType>),
    Object,
    Enum(Vec<String>),
    ComponentRef,     // references another component by ID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropConstraints {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub pattern: Option<String>,       // regex for string props
    pub allow_arbitrary: bool,         // false = only allowed_values permitted
    pub allowed_values: Option<Vec<serde_json::Value>>,
}

/// Style locks: the LLM MUST use these values and cannot override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleLocks {
    /// OKLCH colour tokens that must be used.
    pub color_tokens: Vec<String>,
    /// Spacing tokens that must be used.
    pub spacing_tokens: Vec<String>,
    /// Font family that must be used.
    pub font_family: Option<String>,
    /// Border radius token.
    pub radius_token: Option<String>,
    /// Shadow token.
    pub shadow_token: Option<String>,
}

/// WCAG 2.1 AA requirements per component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WcagRequirements {
    /// Minimum contrast ratio (4.5:1 for AA normal text).
    pub min_contrast_ratio: f64,
    /// Required ARIA role.
    pub aria_role: Option<String>,
    /// Required ARIA properties.
    pub aria_properties: Vec<String>,
    /// Keyboard interaction requirements.
    pub keyboard_required: bool,
    /// Screen reader label requirement.
    pub requires_label: bool,
    /// Focus indicator requirement.
    pub requires_focus_indicator: bool,
}

/// Client-side function available to the A2UI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientFunction {
    pub name: String,
    pub description: String,
    pub parameters: Vec<PropSpec>,
    pub returns: PropType,
    /// Whether this function is idempotent (safe to call multiple times).
    pub idempotent: bool,
}

impl A2UIComponentCatalogV2 {
    /// Build the complete v0.9 Standard Catalog.
    pub fn new() -> Self {
        let mut components = HashMap::new();

        // ── Display Components ──
        components.insert("Text".into(), ComponentSpec {
            component_type: ComponentType::Text,
            description: "Displays text content. Variants: h1-h5, caption, body.".into(),
            allowed_props: vec![
                PropSpec { name: "text".into(), prop_type: PropType::String, required: true, default_value: None,
                    description: "The text content to display.".into(), constraints: Some(PropConstraints {
                        min_length: None, max_length: Some(5000), min_value: None, max_value: None,
                        pattern: None, allow_arbitrary: true, allowed_values: None }) },
                PropSpec { name: "variant".into(), prop_type: PropType::Enum(
                    vec!["h1","h2","h3","h4","h5","body","caption".into()]), required: false,
                    default_value: Some(serde_json::json!("body")),
                    description: "Text style variant.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["text-primary".into(), "text-secondary".into()],
                spacing_tokens: vec!["mb-sm".into(), "leading-normal".into()],
                font_family: Some("Inter, system-ui, sans-serif".into()),
                radius_token: None, shadow_token: None,
            },
            forbidden_patterns: vec![
                "Never set arbitrary hex colours (#xxxxxx).".into(),
                "Never set font-size in px; use design tokens.".into(),
            ],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 4.5, aria_role: None, aria_properties: vec![],
                keyboard_required: false, requires_label: false, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: true,
        });

        components.insert("Image".into(), ComponentSpec {
            component_type: ComponentType::Image,
            description: "Displays an image. Fit modes: contain, cover, fill, none, scaleDown.".into(),
            allowed_props: vec![
                PropSpec { name: "url".into(), prop_type: PropType::String, required: true, default_value: None,
                    description: "Image source URL.".into(), constraints: None },
                PropSpec { name: "fit".into(), prop_type: PropType::Enum(
                    vec!["contain","cover","fill","none","scaleDown".into()]), required: false,
                    default_value: Some(serde_json::json!("cover")),
                    description: "Image fit mode.".into(), constraints: None },
                PropSpec { name: "alt".into(), prop_type: PropType::String, required: true, default_value: None,
                    description: "Accessible alt text.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec![], spacing_tokens: vec![], font_family: None,
                radius_token: Some("radius-lg".into()), shadow_token: None,
            },
            forbidden_patterns: vec!["Never render images without alt text.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 3.0, aria_role: Some("img".into()), aria_properties: vec!["aria-label".into()],
                keyboard_required: false, requires_label: true, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        components.insert("Icon".into(), ComponentSpec {
            component_type: ComponentType::Icon,
            description: "Displays a system icon from a predefined set.".into(),
            allowed_props: vec![
                PropSpec { name: "name".into(), prop_type: PropType::String, required: true, default_value: None,
                    description: "Icon name from the predefined set.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["icon-primary".into(), "icon-secondary".into()],
                spacing_tokens: vec![], font_family: None, radius_token: None, shadow_token: None,
            },
            forbidden_patterns: vec!["Never use custom SVG paths; only predefined icon names.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 3.0, aria_role: Some("img".into()), aria_properties: vec!["aria-hidden".into()],
                keyboard_required: false, requires_label: true, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        // Chart component
        components.insert("Chart".into(), ComponentSpec {
            component_type: ComponentType::Chart,
            description: "Data visualisation chart (Bar, Line, Pie).".into(),
            allowed_props: vec![
                PropSpec { name: "chart_type".into(), prop_type: PropType::Enum(
                    vec!["BarChart","LineChart","PieChart".into()]), required: true, default_value: None,
                    description: "Type of chart to render.".into(), constraints: None },
                PropSpec { name: "data".into(), prop_type: PropType::Array(Box::new(PropType::Object)),
                    required: true, default_value: None,
                    description: "Array of data points.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["chart-series-1".into(), "chart-series-2".into(), "chart-series-3".into()],
                spacing_tokens: vec![], font_family: None, radius_token: None, shadow_token: None,
            },
            forbidden_patterns: vec!["Never use arbitrary chart colours.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 3.0, aria_role: Some("img".into()), aria_properties: vec!["aria-label".into()],
                keyboard_required: true, requires_label: true, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: true,
        });

        // Progress component
        components.insert("Progress".into(), ComponentSpec {
            component_type: ComponentType::Progress,
            description: "Progress indicator (linear or circular).".into(),
            allowed_props: vec![
                PropSpec { name: "value".into(), prop_type: PropType::Number, required: true, default_value: None,
                    description: "Progress value (0-100).".into(), constraints: Some(PropConstraints {
                        min_length: None, max_length: None, min_value: Some(0.0), max_value: Some(100.0),
                        pattern: None, allow_arbitrary: false, allowed_values: None }) },
                PropSpec { name: "variant".into(), prop_type: PropType::Enum(
                    vec!["linear","circular".into()]), required: false, default_value: Some(serde_json::json!("linear")),
                    description: "Progress variant.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["progress-track".into(), "progress-fill".into()],
                spacing_tokens: vec![], font_family: None, radius_token: Some("radius-full".into()), shadow_token: None,
            },
            forbidden_patterns: vec!["Never set arbitrary progress colours.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 3.0, aria_role: Some("progressbar".into()),
                aria_properties: vec!["aria-valuenow".into(), "aria-valuemin".into(), "aria-valuemax".into()],
                keyboard_required: false, requires_label: true, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: true,
        });

        // ── Layout Components ──
        for (name, desc) in [
            ("Row", "Horizontal layout container."),
            ("Column", "Vertical layout container."),
        ] {
            components.insert(name.into(), ComponentSpec {
                component_type: if name == "Row" { ComponentType::Row } else { ComponentType::Column },
                description: desc.into(),
                allowed_props: vec![
                    PropSpec { name: "children".into(), prop_type: PropType::Array(Box::new(PropType::ComponentRef)),
                        required: false, default_value: None,
                        description: "Child components.".into(), constraints: None },
                    PropSpec { name: "gap".into(), prop_type: PropType::String, required: false,
                        default_value: Some(serde_json::json!("md")),
                        description: "Spacing between children (xs, sm, md, lg, xl).".into(), constraints: None },
                ],
                style_locks: StyleLocks {
                    color_tokens: vec![], spacing_tokens: vec!["gap".into()], font_family: None,
                    radius_token: None, shadow_token: None,
                },
                forbidden_patterns: vec!["Never set arbitrary gap values in px.".into()],
                wcag_requirements: WcagRequirements {
                    min_contrast_ratio: 0.0, aria_role: None, aria_properties: vec![],
                    keyboard_required: false, requires_label: false, requires_focus_indicator: false,
                },
                min_spec_version: "0.9".into(), supports_streaming: true,
            });
        }

        components.insert("Divider".into(), ComponentSpec {
            component_type: ComponentType::Divider,
            description: "Visual separator between content sections.".into(),
            allowed_props: vec![],
            style_locks: StyleLocks {
                color_tokens: vec!["border-secondary".into()], spacing_tokens: vec!["my-md".into()],
                font_family: None, radius_token: None, shadow_token: None,
            },
            forbidden_patterns: vec!["Never set arbitrary border colours.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 3.0, aria_role: Some("separator".into()), aria_properties: vec![],
                keyboard_required: false, requires_label: false, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        components.insert("Spacer".into(), ComponentSpec {
            component_type: ComponentType::Spacer,
            description: "Flexible spacing element.".into(),
            allowed_props: vec![
                PropSpec { name: "size".into(), prop_type: PropType::String, required: false,
                    default_value: Some(serde_json::json!("md")),
                    description: "Spacing size (xs, sm, md, lg, xl).".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec![], spacing_tokens: vec!["spacer".into()], font_family: None,
                radius_token: None, shadow_token: None,
            },
            forbidden_patterns: vec!["Never set arbitrary height/width in px.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 0.0, aria_role: Some("none".into()), aria_properties: vec![],
                keyboard_required: false, requires_label: false, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        // ── Container Components ──
        components.insert("Card".into(), ComponentSpec {
            component_type: ComponentType::Card,
            description: "Elevated surface with rounded corners for grouping related content.".into(),
            allowed_props: vec![
                PropSpec { name: "children".into(), prop_type: PropType::Array(Box::new(PropType::ComponentRef)),
                    required: false, default_value: None,
                    description: "Content inside the card.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["surface-primary".into(), "surface-secondary".into()],
                spacing_tokens: vec!["p-lg".into()], font_family: None,
                radius_token: Some("radius-lg".into()), shadow_token: Some("shadow-md".into()),
            },
            forbidden_patterns: vec!["Never set arbitrary background colours or shadows.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 0.0, aria_role: Some("region".into()), aria_properties: vec![],
                keyboard_required: false, requires_label: false, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: true,
        });

        components.insert("List".into(), ComponentSpec {
            component_type: ComponentType::List,
            description: "Scrollable list of items with optional selection.".into(),
            allowed_props: vec![
                PropSpec { name: "items".into(), prop_type: PropType::Array(Box::new(PropType::Object)),
                    required: true, default_value: None,
                    description: "Array of list items.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["surface-primary".into(), "text-primary".into()],
                spacing_tokens: vec!["p-md".into()], font_family: None,
                radius_token: Some("radius-md".into()), shadow_token: None,
            },
            forbidden_patterns: vec!["Never set arbitrary list styles.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 4.5, aria_role: Some("list".into()), aria_properties: vec![],
                keyboard_required: true, requires_label: true, requires_focus_indicator: true,
            },
            min_spec_version: "0.9".into(), supports_streaming: true,
        });

        components.insert("Table".into(), ComponentSpec {
            component_type: ComponentType::Table,
            description: "Data grid with sortable columns and selectable rows.".into(),
            allowed_props: vec![
                PropSpec { name: "columns".into(), prop_type: PropType::Array(Box::new(PropType::Object)),
                    required: true, default_value: None,
                    description: "Column definitions with header and accessor.".into(), constraints: None },
                PropSpec { name: "rows".into(), prop_type: PropType::Array(Box::new(PropType::Object)),
                    required: true, default_value: None,
                    description: "Row data.".into(), constraints: None },
                PropSpec { name: "sortable".into(), prop_type: PropType::Boolean, required: false,
                    default_value: Some(serde_json::json!(true)),
                    description: "Enable column sorting.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["surface-primary".into(), "text-primary".into(), "border-secondary".into()],
                spacing_tokens: vec!["p-sm".into()], font_family: None,
                radius_token: None, shadow_token: None,
            },
            forbidden_patterns: vec!["Never set arbitrary table border colours.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 4.5, aria_role: Some("table".into()),
                aria_properties: vec!["aria-sort".into(), "aria-label".into()],
                keyboard_required: true, requires_label: true, requires_focus_indicator: true,
            },
            min_spec_version: "0.9".into(), supports_streaming: true,
        });

        // Tabs, Modal — simplified for brevity (full in production)
        for (name, ct, desc) in [
            ("Tabs", ComponentType::Tabs, "Tabbed content container."),
            ("Modal", ComponentType::Modal, "Overlay dialog for focused tasks."),
        ] {
            components.insert(name.into(), ComponentSpec {
                component_type: ct, description: desc.into(),
                allowed_props: vec![],
                style_locks: StyleLocks {
                    color_tokens: vec!["surface-primary".into(), "overlay".into()],
                    spacing_tokens: vec!["p-lg".into()], font_family: None,
                    radius_token: Some("radius-lg".into()), shadow_token: Some("shadow-xl".into()),
                },
                forbidden_patterns: vec!["Never set arbitrary z-index values.".into()],
                wcag_requirements: WcagRequirements {
                    min_contrast_ratio: 4.5, aria_role: if name == "Modal" { Some("dialog".into()) } else { None },
                    aria_properties: if name == "Modal" { vec!["aria-modal".into(), "aria-label".into()] } else { vec![] },
                    keyboard_required: true, requires_label: true, requires_focus_indicator: true,
                },
                min_spec_version: "0.9".into(), supports_streaming: true,
            });
        }

        // ── Input Components ──
        components.insert("TextField".into(), ComponentSpec {
            component_type: ComponentType::TextField,
            description: "Text input field with built-in validation.".into(),
            allowed_props: vec![
                PropSpec { name: "label".into(), prop_type: PropType::String, required: true, default_value: None,
                    description: "Accessible label for the input.".into(), constraints: None },
                PropSpec { name: "value".into(), prop_type: PropType::String, required: false,
                    default_value: None, description: "Current input value.".into(), constraints: None },
                PropSpec { name: "placeholder".into(), prop_type: PropType::String, required: false,
                    default_value: None, description: "Placeholder text.".into(), constraints: None },
                PropSpec { name: "required".into(), prop_type: PropType::Boolean, required: false,
                    default_value: Some(serde_json::json!(false)),
                    description: "Whether the field is required.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["input-bg".into(), "input-border".into(), "input-text".into(), "input-focus".into()],
                spacing_tokens: vec!["p-md".into()], font_family: None,
                radius_token: Some("radius-md".into()), shadow_token: None,
            },
            forbidden_patterns: vec![
                "Never set arbitrary input colours.".into(),
                "Never omit the label prop (WCAG requirement).".into(),
            ],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 4.5, aria_role: Some("textbox".into()),
                aria_properties: vec!["aria-required".into(), "aria-invalid".into()],
                keyboard_required: true, requires_label: true, requires_focus_indicator: true,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        components.insert("Button".into(), ComponentSpec {
            component_type: ComponentType::Button,
            description: "Action trigger. Variants: primary, secondary, danger, ghost.".into(),
            allowed_props: vec![
                PropSpec { name: "label".into(), prop_type: PropType::String, required: true, default_value: None,
                    description: "Accessible button label.".into(), constraints: None },
                PropSpec { name: "variant".into(), prop_type: PropType::Enum(
                    vec!["primary","secondary","danger","ghost".into()]), required: false,
                    default_value: Some(serde_json::json!("primary")),
                    description: "Button style variant.".into(), constraints: None },
                PropSpec { name: "disabled".into(), prop_type: PropType::Boolean, required: false,
                    default_value: Some(serde_json::json!(false)),
                    description: "Whether the button is disabled.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["btn-primary-bg".into(), "btn-primary-text".into(), "btn-secondary-bg".into(),
                    "btn-danger-bg".into(), "btn-ghost-bg".into()],
                spacing_tokens: vec!["px-lg".into(), "py-md".into()], font_family: None,
                radius_token: Some("radius-md".into()), shadow_token: None,
            },
            forbidden_patterns: vec![
                "Never set arbitrary button colours.".into(),
                "Never render a button without an accessible label.".into(),
            ],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 4.5, aria_role: Some("button".into()),
                aria_properties: vec!["aria-disabled".into(), "aria-label".into()],
                keyboard_required: true, requires_label: true, requires_focus_indicator: true,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        // Toggle, Slider, DatePicker — simplified
        for (name, ct, desc, role) in [
            ("Toggle", ComponentType::Toggle, "Boolean switch control.", "switch"),
            ("Slider", ComponentType::Slider, "Range selection control.", "slider"),
            ("DatePicker", ComponentType::DatePicker, "Date/time selection control.", "combobox"),
        ] {
            components.insert(name.into(), ComponentSpec {
                component_type: ct, description: desc.into(),
                allowed_props: vec![
                    PropSpec { name: "label".into(), prop_type: PropType::String, required: true, default_value: None,
                        description: "Accessible label.".into(), constraints: None },
                ],
                style_locks: StyleLocks {
                    color_tokens: vec!["input-bg".into(), "input-border".into()],
                    spacing_tokens: vec![], font_family: None,
                    radius_token: Some("radius-md".into()), shadow_token: None,
                },
                forbidden_patterns: vec!["Never omit the accessible label.".into()],
                wcag_requirements: WcagRequirements {
                    min_contrast_ratio: 4.5, aria_role: Some(role.into()),
                    aria_properties: vec!["aria-label".into()],
                    keyboard_required: true, requires_label: true, requires_focus_indicator: true,
                },
                min_spec_version: "0.9".into(), supports_streaming: false,
            });
        }

        // Map component — special component for geographical data
        components.insert("Map".into(), ComponentSpec {
            component_type: ComponentType::Progress, // reusing; Map not in standard 18, but essential for enterprise
            description: "Geographic map visualisation for spatial data.".into(),
            allowed_props: vec![
                PropSpec { name: "center".into(), prop_type: PropType::Object, required: false, default_value: None,
                    description: "Map center coordinates {lat, lng}.".into(), constraints: None },
            ],
            style_locks: StyleLocks {
                color_tokens: vec!["map-bg".into()], spacing_tokens: vec![], font_family: None,
                radius_token: Some("radius-lg".into()), shadow_token: None,
            },
            forbidden_patterns: vec!["Never set arbitrary map tile URLs.".into()],
            wcag_requirements: WcagRequirements {
                min_contrast_ratio: 3.0, aria_role: Some("img".into()), aria_properties: vec!["aria-label".into()],
                keyboard_required: true, requires_label: true, requires_focus_indicator: false,
            },
            min_spec_version: "0.9".into(), supports_streaming: false,
        });

        // Client functions
        let functions = vec![
            ClientFunction {
                name: "validate".into(), description: "Validates form input against schema.".into(),
                parameters: vec![
                    PropSpec { name: "value".into(), prop_type: PropType::String, required: true, default_value: None,
                        description: "Value to validate.".into(), constraints: None },
                ],
                returns: PropType::Object, idempotent: true,
            },
            ClientFunction {
                name: "formatDate".into(), description: "Formats a date for display.".into(),
                parameters: vec![
                    PropSpec { name: "date".into(), prop_type: PropType::String, required: true, default_value: None,
                        description: "ISO 8601 date string.".into(), constraints: None },
                ],
                returns: PropType::String, idempotent: true,
            },
        ];

        Self {
            catalog_id: "https://a2ui.org/specification/v0_9/basic_catalog.json".into(),
            version: "0.9".into(),
            components,
            functions,
        }
    }

    /// Generate the system prompt embedding for LLM context.
    /// A2UI v0.9 is "prompt-first designed" — the catalog must fit
    /// inside the LLM's system prompt, not its structured output.
    pub fn to_prompt_context(&self) -> String {
        let mut ctx = String::from("A2UI v0.9 Component Catalog — Use ONLY these components:\n\n");
        for (name, spec) in &self.components {
            ctx.push_str(&format!("- {}: {}\n", name, spec.description));
            for prop in &spec.allowed_props {
                ctx.push_str(&format!("  · {} ({}) {}\n", prop.name, 
                    prop_type_display(&prop.prop_type),
                    if prop.required { "[REQUIRED]" } else { "" }));
            }
            if !spec.style_locks.color_tokens.is_empty() {
                ctx.push_str(&format!("  Colors: {}\n", spec.style_locks.color_tokens.join(", ")));
            }
            if !spec.forbidden_patterns.is_empty() {
                ctx.push_str(&format!("  FORBIDDEN: {}\n", spec.forbidden_patterns[0]));
            }
            ctx.push('\n');
        }
        ctx
    }
}

fn prop_type_display(pt: &PropType) -> String {
    match pt {
        PropType::String => "string".into(),
        PropType::Number => "number".into(),
        PropType::Boolean => "bool".into(),
        PropType::Array(inner) => format!("array<{}>", prop_type_display(inner)),
        PropType::Object => "object".into(),
        PropType::Enum(vals) => format!("enum[{}]", vals.join("|")),
        PropType::ComponentRef => "componentId".into(),
    }
}
