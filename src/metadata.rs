use taurino_core::{anyhow, image::Image, serde::Deserialize};

/// Application metadata for the [`PredefinedMenuItem::about`].
#[derive(Debug, Clone, Default)]
pub struct AboutMetadata<'a> {
    /// Sets the application name.
    pub name: Option<String>,
    /// The application version.
    pub version: Option<String>,
    /// The short version, e.g. "1.0".
    ///
    /// ## Platform-specific
    ///
    /// - **Windows / Linux:** Appended to the end of `version` in parentheses.
    pub short_version: Option<String>,
    /// The authors of the application.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub authors: Option<Vec<String>>,
    /// Application comments.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub comments: Option<String>,
    /// The copyright of the application.
    pub copyright: Option<String>,
    /// The license of the application.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub license: Option<String>,
    /// The application website.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub website: Option<String>,
    /// The website label.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub website_label: Option<String>,
    /// The credits.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows / Linux:** Unsupported.
    pub credits: Option<String>,
    /// The application icon.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** Unsupported.
    pub icon: Option<Image<'a>>,
}

/// A builder type for [`AboutMetadata`].
#[derive(Clone, Debug, Default)]
pub struct AboutMetadataBuilder<'a>(AboutMetadata<'a>);

impl<'a> AboutMetadataBuilder<'a> {
    /// Create a new about metadata builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the application name.
    pub fn name<S: Into<String>>(mut self, name: Option<S>) -> Self {
        self.0.name = name.map(|s| s.into());
        self
    }
    /// Sets the application version.
    pub fn version<S: Into<String>>(mut self, version: Option<S>) -> Self {
        self.0.version = version.map(|s| s.into());
        self
    }
    /// Sets the short version, e.g. "1.0".
    ///
    /// ## Platform-specific
    ///
    /// - **Windows / Linux:** Appended to the end of `version` in parentheses.
    pub fn short_version<S: Into<String>>(mut self, short_version: Option<S>) -> Self {
        self.0.short_version = short_version.map(|s| s.into());
        self
    }
    /// Sets the authors of the application.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub fn authors(mut self, authors: Option<Vec<String>>) -> Self {
        self.0.authors = authors;
        self
    }
    /// Application comments.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub fn comments<S: Into<String>>(mut self, comments: Option<S>) -> Self {
        self.0.comments = comments.map(|s| s.into());
        self
    }
    /// Sets the copyright of the application.
    pub fn copyright<S: Into<String>>(mut self, copyright: Option<S>) -> Self {
        self.0.copyright = copyright.map(|s| s.into());
        self
    }
    /// Sets the license of the application.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub fn license<S: Into<String>>(mut self, license: Option<S>) -> Self {
        self.0.license = license.map(|s| s.into());
        self
    }
    /// Sets the application website.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub fn website<S: Into<String>>(mut self, website: Option<S>) -> Self {
        self.0.website = website.map(|s| s.into());
        self
    }
    /// Sets the website label.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Unsupported.
    pub fn website_label<S: Into<String>>(mut self, website_label: Option<S>) -> Self {
        self.0.website_label = website_label.map(|s| s.into());
        self
    }
    /// Sets the credits.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows / Linux:** Unsupported.
    pub fn credits<S: Into<String>>(mut self, credits: Option<S>) -> Self {
        self.0.credits = credits.map(|s| s.into());
        self
    }
    /// Sets the application icon.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** Unsupported.
    pub fn icon(mut self, icon: Option<Image<'a>>) -> Self {
        self.0.icon = icon;
        self
    }

    /// Construct the final [`AboutMetadata`]
    pub fn build(self) -> AboutMetadata<'a> {
        self.0
    }
}

impl TryFrom<AboutMetadata<'_>> for taurino_core::muda::AboutMetadata {
    type Error = anyhow::Error;

    fn try_from(value: AboutMetadata<'_>) -> Result<Self, Self::Error> {
        let icon = match value.icon {
            Some(i) => Some(i.try_into()?),
            None => None,
        };

        Ok(Self {
            authors: value.authors,
            name: value.name,
            version: value.version,
            short_version: value.short_version,
            comments: value.comments,
            copyright: value.copyright,
            license: value.license,
            website: value.website,
            website_label: value.website_label,
            credits: value.credits,
            icon,
        })
    }
}

/// A native Icon to be used for the menu item
///
/// ## Platform-specific:
///
/// - **Windows / Linux**: Unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(crate = "taurino_core::serde")]
pub enum NativeIcon {
    /// An add item template image.
    Add,
    /// Advanced preferences toolbar icon for the preferences window.
    Advanced,
    /// A Bluetooth template image.
    Bluetooth,
    /// Bookmarks image suitable for a template.
    Bookmarks,
    /// A caution image.
    Caution,
    /// A color panel toolbar icon.
    ColorPanel,
    /// A column view mode template image.
    ColumnView,
    /// A computer icon.
    Computer,
    /// An enter full-screen mode template image.
    EnterFullScreen,
    /// Permissions for all users.
    Everyone,
    /// An exit full-screen mode template image.
    ExitFullScreen,
    /// A cover flow view mode template image.
    FlowView,
    /// A folder image.
    Folder,
    /// A burnable folder icon.
    FolderBurnable,
    /// A smart folder icon.
    FolderSmart,
    /// A link template image.
    FollowLinkFreestanding,
    /// A font panel toolbar icon.
    FontPanel,
    /// A `go back` template image.
    GoLeft,
    /// A `go forward` template image.
    GoRight,
    /// Home image suitable for a template.
    Home,
    /// An iChat Theater template image.
    IChatTheater,
    /// An icon view mode template image.
    IconView,
    /// An information toolbar icon.
    Info,
    /// A template image used to denote invalid data.
    InvalidDataFreestanding,
    /// A generic left-facing triangle template image.
    LeftFacingTriangle,
    /// A list view mode template image.
    ListView,
    /// A locked padlock template image.
    LockLocked,
    /// An unlocked padlock template image.
    LockUnlocked,
    /// A horizontal dash, for use in menus.
    MenuMixedState,
    /// A check mark template image, for use in menus.
    MenuOnState,
    /// A MobileMe icon.
    MobileMe,
    /// A drag image for multiple items.
    MultipleDocuments,
    /// A network icon.
    Network,
    /// A path button template image.
    Path,
    /// General preferences toolbar icon for the preferences window.
    PreferencesGeneral,
    /// A Quick Look template image.
    QuickLook,
    /// A refresh template image.
    RefreshFreestanding,
    /// A refresh template image.
    Refresh,
    /// A remove item template image.
    Remove,
    /// A reveal contents template image.
    RevealFreestanding,
    /// A generic right-facing triangle template image.
    RightFacingTriangle,
    /// A share view template image.
    Share,
    /// A slideshow template image.
    Slideshow,
    /// A badge for a `smart` item.
    SmartBadge,
    /// Small green indicator, similar to iChat's available image.
    StatusAvailable,
    /// Small clear indicator.
    StatusNone,
    /// Small yellow indicator, similar to iChat's idle image.
    StatusPartiallyAvailable,
    /// Small red indicator, similar to iChat's unavailable image.
    StatusUnavailable,
    /// A stop progress template image.
    StopProgressFreestanding,
    /// A stop progress button template image.
    StopProgress,
    /// An image of the empty trash can.
    TrashEmpty,
    /// An image of the full trash can.
    TrashFull,
    /// Permissions for a single user.
    User,
    /// User account toolbar icon for the preferences window.
    UserAccounts,
    /// Permissions for a group of users.
    UserGroup,
    /// Permissions for guests.
    UserGuest,
}

impl From<NativeIcon> for taurino_core::muda::NativeIcon {
    fn from(value: NativeIcon) -> Self {
        match value {
            NativeIcon::Add => taurino_core::muda::NativeIcon::Add,
            NativeIcon::Advanced => taurino_core::muda::NativeIcon::Advanced,
            NativeIcon::Bluetooth => taurino_core::muda::NativeIcon::Bluetooth,
            NativeIcon::Bookmarks => taurino_core::muda::NativeIcon::Bookmarks,
            NativeIcon::Caution => taurino_core::muda::NativeIcon::Caution,
            NativeIcon::ColorPanel => taurino_core::muda::NativeIcon::ColorPanel,
            NativeIcon::ColumnView => taurino_core::muda::NativeIcon::ColumnView,
            NativeIcon::Computer => taurino_core::muda::NativeIcon::Computer,
            NativeIcon::EnterFullScreen => taurino_core::muda::NativeIcon::EnterFullScreen,
            NativeIcon::Everyone => taurino_core::muda::NativeIcon::Everyone,
            NativeIcon::ExitFullScreen => taurino_core::muda::NativeIcon::ExitFullScreen,
            NativeIcon::FlowView => taurino_core::muda::NativeIcon::FlowView,
            NativeIcon::Folder => taurino_core::muda::NativeIcon::Folder,
            NativeIcon::FolderBurnable => taurino_core::muda::NativeIcon::FolderBurnable,
            NativeIcon::FolderSmart => taurino_core::muda::NativeIcon::FolderSmart,
            NativeIcon::FollowLinkFreestanding => taurino_core::muda::NativeIcon::FollowLinkFreestanding,
            NativeIcon::FontPanel => taurino_core::muda::NativeIcon::FontPanel,
            NativeIcon::GoLeft => taurino_core::muda::NativeIcon::GoLeft,
            NativeIcon::GoRight => taurino_core::muda::NativeIcon::GoRight,
            NativeIcon::Home => taurino_core::muda::NativeIcon::Home,
            NativeIcon::IChatTheater => taurino_core::muda::NativeIcon::IChatTheater,
            NativeIcon::IconView => taurino_core::muda::NativeIcon::IconView,
            NativeIcon::Info => taurino_core::muda::NativeIcon::Info,
            NativeIcon::InvalidDataFreestanding => taurino_core::muda::NativeIcon::InvalidDataFreestanding,
            NativeIcon::LeftFacingTriangle => taurino_core::muda::NativeIcon::LeftFacingTriangle,
            NativeIcon::ListView => taurino_core::muda::NativeIcon::ListView,
            NativeIcon::LockLocked => taurino_core::muda::NativeIcon::LockLocked,
            NativeIcon::LockUnlocked => taurino_core::muda::NativeIcon::LockUnlocked,
            NativeIcon::MenuMixedState => taurino_core::muda::NativeIcon::MenuMixedState,
            NativeIcon::MenuOnState => taurino_core::muda::NativeIcon::MenuOnState,
            NativeIcon::MobileMe => taurino_core::muda::NativeIcon::MobileMe,
            NativeIcon::MultipleDocuments => taurino_core::muda::NativeIcon::MultipleDocuments,
            NativeIcon::Network => taurino_core::muda::NativeIcon::Network,
            NativeIcon::Path => taurino_core::muda::NativeIcon::Path,
            NativeIcon::PreferencesGeneral => taurino_core::muda::NativeIcon::PreferencesGeneral,
            NativeIcon::QuickLook => taurino_core::muda::NativeIcon::QuickLook,
            NativeIcon::RefreshFreestanding => taurino_core::muda::NativeIcon::RefreshFreestanding,
            NativeIcon::Refresh => taurino_core::muda::NativeIcon::Refresh,
            NativeIcon::Remove => taurino_core::muda::NativeIcon::Remove,
            NativeIcon::RevealFreestanding => taurino_core::muda::NativeIcon::RevealFreestanding,
            NativeIcon::RightFacingTriangle => taurino_core::muda::NativeIcon::RightFacingTriangle,
            NativeIcon::Share => taurino_core::muda::NativeIcon::Share,
            NativeIcon::Slideshow => taurino_core::muda::NativeIcon::Slideshow,
            NativeIcon::SmartBadge => taurino_core::muda::NativeIcon::SmartBadge,
            NativeIcon::StatusAvailable => taurino_core::muda::NativeIcon::StatusAvailable,
            NativeIcon::StatusNone => taurino_core::muda::NativeIcon::StatusNone,
            NativeIcon::StatusPartiallyAvailable => taurino_core::muda::NativeIcon::StatusPartiallyAvailable,
            NativeIcon::StatusUnavailable => taurino_core::muda::NativeIcon::StatusUnavailable,
            NativeIcon::StopProgressFreestanding => taurino_core::muda::NativeIcon::StopProgressFreestanding,
            NativeIcon::StopProgress => taurino_core::muda::NativeIcon::StopProgress,
            NativeIcon::TrashEmpty => taurino_core::muda::NativeIcon::TrashEmpty,
            NativeIcon::TrashFull => taurino_core::muda::NativeIcon::TrashFull,
            NativeIcon::User => taurino_core::muda::NativeIcon::User,
            NativeIcon::UserAccounts => taurino_core::muda::NativeIcon::UserAccounts,
            NativeIcon::UserGroup => taurino_core::muda::NativeIcon::UserGroup,
            NativeIcon::UserGuest => taurino_core::muda::NativeIcon::UserGuest,
        }
    }
}
