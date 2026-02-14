//! Configure a renderer.
use crate::core::{Font, Pixels};
use crate::graphics::{self, Antialiasing};

/// The settings of a [`Renderer`].
///
/// [`Renderer`]: crate::Renderer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    /// The present mode of the [`Renderer`].
    ///
    /// [`Renderer`]: crate::Renderer
    pub present_mode: wgpu::PresentMode,

    /// The preferred surface [`wgpu::TextureFormat`].
    ///
    /// If `Some`, the compositor will use this format when supported by the
    /// surface. Use `Rgba16Float` on macOS for HDR/EDR output.
    ///
    /// If `None`, the compositor selects a format automatically.
    pub format: Option<wgpu::TextureFormat>,

    /// The graphics backends to use.
    pub backends: wgpu::Backends,

    /// The default [`Font`] to use.
    pub default_font: Font,

    /// The default size of text.
    ///
    /// By default, it will be set to `16.0`.
    pub default_text_size: Pixels,

    /// The antialiasing strategy that will be used for triangle primitives.
    ///
    /// By default, it is `None`.
    pub antialiasing: Option<Antialiasing>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            present_mode: wgpu::PresentMode::AutoVsync,
            format: None,
            backends: wgpu::Backends::all(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: None,
        }
    }
}

impl From<graphics::Settings> for Settings {
    fn from(settings: graphics::Settings) -> Self {
        Self {
            present_mode: if settings.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            },
            default_font: settings.default_font,
            default_text_size: settings.default_text_size,
            antialiasing: settings.antialiasing,
            ..Settings::default()
        }
    }
}

/// Obtains a [`wgpu::TextureFormat`] from the current environment
/// configuration, if set.
///
/// The value returned by this function can be changed by setting
/// the `ICED_FORMAT` env variable. The possible values are:
///
/// - `"rgba16float"` → [`wgpu::TextureFormat::Rgba16Float`] (HDR/EDR on macOS)
/// - `"bgra8unormsrgb"` → [`wgpu::TextureFormat::Bgra8UnormSrgb`]
/// - `"bgra8unorm"` → [`wgpu::TextureFormat::Bgra8Unorm`]
/// - `"rgba8unormsrgb"` → [`wgpu::TextureFormat::Rgba8UnormSrgb`]
/// - `"rgba8unorm"` → [`wgpu::TextureFormat::Rgba8Unorm`]
pub fn format_from_env() -> Option<wgpu::TextureFormat> {
    let format = std::env::var("ICED_FORMAT").ok()?;

    match format.to_lowercase().as_str() {
        "rgba16float" => Some(wgpu::TextureFormat::Rgba16Float),
        "bgra8unormsrgb" => Some(wgpu::TextureFormat::Bgra8UnormSrgb),
        "bgra8unorm" => Some(wgpu::TextureFormat::Bgra8Unorm),
        "rgba8unormsrgb" => Some(wgpu::TextureFormat::Rgba8UnormSrgb),
        "rgba8unorm" => Some(wgpu::TextureFormat::Rgba8Unorm),
        _ => None,
    }
}

/// Obtains a [`wgpu::PresentMode`] from the current environment
/// configuration, if set.
///
/// The value returned by this function can be changed by setting
/// the `ICED_PRESENT_MODE` env variable. The possible values are:
///
/// - `"vsync"` → [`wgpu::PresentMode::AutoVsync`]
/// - `"no_vsync"` → [`wgpu::PresentMode::AutoNoVsync`]
/// - `"immediate"` → [`wgpu::PresentMode::Immediate`]
/// - `"fifo"` → [`wgpu::PresentMode::Fifo`]
/// - `"fifo_relaxed"` → [`wgpu::PresentMode::FifoRelaxed`]
/// - `"mailbox"` → [`wgpu::PresentMode::Mailbox`]
pub fn present_mode_from_env() -> Option<wgpu::PresentMode> {
    let present_mode = std::env::var("ICED_PRESENT_MODE").ok()?;

    match present_mode.to_lowercase().as_str() {
        "vsync" => Some(wgpu::PresentMode::AutoVsync),
        "no_vsync" => Some(wgpu::PresentMode::AutoNoVsync),
        "immediate" => Some(wgpu::PresentMode::Immediate),
        "fifo" => Some(wgpu::PresentMode::Fifo),
        "fifo_relaxed" => Some(wgpu::PresentMode::FifoRelaxed),
        "mailbox" => Some(wgpu::PresentMode::Mailbox),
        _ => None,
    }
}
