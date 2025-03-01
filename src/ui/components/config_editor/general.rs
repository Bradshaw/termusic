/**
 * MIT License
 *
 * tuifeed - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use crate::config::{LastPosition, SeekStep, Settings};
use crate::ui::{ConfigEditorMsg, Msg};

use tui_realm_stdlib::{Input, Radio};
// use tuirealm::props::{Alignment, BorderSides, BorderType, Borders, Color, TableBuilder, TextSpan};
use crate::ui::components::Alignment as XywhAlign;
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent, NoUserEvent},
    Component, Event, MockComponent,
};

#[derive(MockComponent)]
pub struct MusicDir {
    component: Input,
    config: Settings,
}

impl MusicDir {
    pub fn new(config: &Settings) -> Self {
        let mut music_dir = String::new();
        for m in &config.music_dir {
            music_dir.push_str(m.as_str());
            music_dir.push(';');
        }
        let _ = music_dir.remove(music_dir.len() - 1);
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightGreen),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightGreen),
                )
                .input_type(InputType::Text)
                .placeholder("~/Music", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title(
                    " Root Music Directory:(use ; to separate) ",
                    Alignment::Left,
                )
                .value(music_dir),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for MusicDir {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_input_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::MusicDirBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::MusicDirBlurUp),
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_input_ev(
    component: &mut dyn Component<Msg, NoUserEvent>,
    ev: Event<NoUserEvent>,
    config: &Settings,
    on_key_down: Msg,
    on_key_up: Msg,
) -> Option<Msg> {
    match ev {
        // Global Hotkeys
        Event::Keyboard(keyevent) if keyevent == config.keys.config_save.key_event() => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::CloseOk))
        }
        Event::Keyboard(KeyEvent {
            code: Key::Down, ..
        }) => Some(on_key_down),
        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(on_key_up),
        Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::ChangeLayout))
        }
        Event::Keyboard(keyevent) if keyevent == config.keys.global_esc.key_event() => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::CloseCancel))
        }

        // Local Hotkeys
        Event::Keyboard(KeyEvent {
            code: Key::Left, ..
        }) => {
            component.perform(Cmd::Move(Direction::Left));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Right, ..
        }) => {
            component.perform(Cmd::Move(Direction::Right));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Home, ..
        }) => {
            component.perform(Cmd::GoTo(Position::Begin));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
            component.perform(Cmd::GoTo(Position::End));
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Delete, ..
        }) => {
            component.perform(Cmd::Cancel);
            Some(Msg::None)
        }
        Event::Keyboard(KeyEvent {
            code: Key::Backspace,
            ..
        }) => {
            component.perform(Cmd::Delete);
            Some(Msg::None)
        }

        Event::Keyboard(KeyEvent {
            code: Key::Char(ch),
            ..
        }) => {
            component.perform(Cmd::Type(ch));
            Some(Msg::ConfigEditor(ConfigEditorMsg::ConfigChanged))
        }

        _ => None,
    }
}

#[derive(MockComponent)]
pub struct ExitConfirmation {
    component: Radio,
    config: Settings,
}

impl ExitConfirmation {
    pub fn new(config: &Settings) -> Self {
        let enabled = config.enable_exit_confirmation;
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Yes", "No"])
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .rewind(true)
                .title(" Show exit confirmation? ", Alignment::Left)
                .value(usize::from(!enabled)),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExitConfirmation {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_radio_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::ExitConfirmationBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::ExitConfirmationBlurUp),
        )
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_radio_ev(
    component: &mut dyn Component<Msg, NoUserEvent>,
    ev: Event<NoUserEvent>,
    config: &Settings,
    on_key_down: Msg,
    on_key_up: Msg,
) -> Option<Msg> {
    match ev {
        // Global Hotkeys
        Event::Keyboard(keyevent) if keyevent == config.keys.config_save.key_event() => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::CloseOk))
        }
        Event::Keyboard(KeyEvent {
            code: Key::Down, ..
        }) => Some(on_key_down),
        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(on_key_up),
        Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::ChangeLayout))
        }
        Event::Keyboard(keyevent) if keyevent == config.keys.global_down.key_event() => {
            Some(on_key_down)
        }
        Event::Keyboard(keyevent) if keyevent == config.keys.global_up.key_event() => {
            Some(on_key_up)
        }
        Event::Keyboard(keyevent) if keyevent == config.keys.global_quit.key_event() => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::CloseCancel))
        }
        Event::Keyboard(keyevent) if keyevent == config.keys.global_esc.key_event() => {
            Some(Msg::ConfigEditor(ConfigEditorMsg::CloseCancel))
        }

        // Local Hotkeys
        Event::Keyboard(KeyEvent {
            code: Key::Left, ..
        }) => {
            component.perform(Cmd::Move(Direction::Left));
            Some(Msg::ConfigEditor(ConfigEditorMsg::ConfigChanged))
        }
        Event::Keyboard(KeyEvent {
            code: Key::Right, ..
        }) => {
            component.perform(Cmd::Move(Direction::Right));
            Some(Msg::ConfigEditor(ConfigEditorMsg::ConfigChanged))
        }

        _ => None,
    }
}

#[derive(MockComponent)]
pub struct PlaylistDisplaySymbol {
    component: Radio,
    config: Settings,
}

impl PlaylistDisplaySymbol {
    pub fn new(config: &Settings) -> Self {
        let enabled = config.enable_exit_confirmation;
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Yes", "No"])
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .rewind(true)
                .title(" Display symbol in playlist title? ", Alignment::Left)
                .value(usize::from(!enabled)),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for PlaylistDisplaySymbol {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_radio_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::PlaylistDisplaySymbolBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::PlaylistDisplaySymbolBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct PlaylistRandomTrack {
    component: Input,
    config: Settings,
}

impl PlaylistRandomTrack {
    pub fn new(config: &Settings) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .input_type(InputType::UnsignedInteger)
                .invalid_style(Style::default().fg(Color::Red))
                .placeholder("20", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title(" Playlist Select Random Track Quantity: ", Alignment::Left)
                .value(config.playlist_select_random_track_quantity.to_string()),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for PlaylistRandomTrack {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_input_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::PlaylistRandomTrackBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::PlaylistRandomTrackBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct PlaylistRandomAlbum {
    component: Input,
    config: Settings,
}

impl PlaylistRandomAlbum {
    pub fn new(config: &Settings) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .input_type(InputType::UnsignedInteger)
                .invalid_style(Style::default().fg(Color::Red))
                .placeholder("1", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title(
                    " Playlist Select Random Album with tracks no less than: ",
                    Alignment::Left,
                )
                .value(config.playlist_select_random_album_quantity.to_string()),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for PlaylistRandomAlbum {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_input_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::PlaylistRandomAlbumBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::PlaylistRandomAlbumBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct PodcastDir {
    component: Input,
    config: Settings,
}

impl PodcastDir {
    pub fn new(config: &Settings) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .input_type(InputType::Text)
                .invalid_style(Style::default().fg(Color::Red))
                .placeholder(
                    "~/Music/podcast",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title(" Podcast Download Directory: ", Alignment::Left)
                .value(&config.podcast_dir),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for PodcastDir {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_input_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::PodcastDirBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::PodcastDirBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct PodcastSimulDownload {
    component: Input,
    config: Settings,
}

impl PodcastSimulDownload {
    pub fn new(config: &Settings) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .input_type(InputType::UnsignedInteger)
                .invalid_style(Style::default().fg(Color::Red))
                .placeholder(
                    "between 1 ~ 5 suggested",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title(" Podcast Simultanious Download: ", Alignment::Left)
                .value(format!("{}", config.podcast_simultanious_download)),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for PodcastSimulDownload {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_input_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::PodcastSimulDownloadBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::PodcastSimulDownloadBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct PodcastMaxRetries {
    component: Input,
    config: Settings,
}

impl PodcastMaxRetries {
    pub fn new(config: &Settings) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .input_type(InputType::UnsignedInteger)
                .invalid_style(Style::default().fg(Color::Red))
                .placeholder(
                    "between 1 ~ 5 suggested",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title(" Podcast Download Max Retries: ", Alignment::Left)
                .value(format!("{}", config.podcast_max_retries)),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for PodcastMaxRetries {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_input_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::PodcastMaxRetriesBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::PodcastMaxRetriesBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct AlbumPhotoAlign {
    component: Radio,
    config: Settings,
}

impl AlbumPhotoAlign {
    pub fn new(config: &Settings) -> Self {
        let align = match config.album_photo_xywh.align {
            XywhAlign::BottomRight => 0,
            XywhAlign::BottomLeft => 1,
            XywhAlign::TopRight => 2,
            XywhAlign::TopLeft => 3,
        };
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["BottomRight", "BottomLeft", "TopRight", "TopLeft"])
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .rewind(true)
                .title(" Album Photo Align: ", Alignment::Left)
                .value(align),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for AlbumPhotoAlign {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_radio_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::AlbumPhotoAlignBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::AlbumPhotoAlignBlurUp),
        )
    }
}

#[derive(MockComponent)]
pub struct SaveLastPosition {
    component: Radio,
    config: Settings,
}

impl SaveLastPosition {
    pub fn new(config: &Settings) -> Self {
        let save_last_position = match config.remember_last_played_position {
            LastPosition::Auto => 0,
            LastPosition::No => 1,
            LastPosition::Yes => 2,
        };
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Auto", "No", "Yes"])
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .rewind(true)
                .title(" Remember last played position: ", Alignment::Left)
                .value(save_last_position),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for SaveLastPosition {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_radio_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::SaveLastPositionBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::SaveLastPosotionBlurUp),
        )
    }
}
#[derive(MockComponent)]
pub struct ConfigSeekStep {
    component: Radio,
    config: Settings,
}

impl ConfigSeekStep {
    pub fn new(config: &Settings) -> Self {
        let seek_step = match config.seek_step {
            SeekStep::Auto => 0,
            SeekStep::Short => 1,
            SeekStep::Long => 2,
        };
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(
                            config
                                .style_color_symbol
                                .library_border()
                                .unwrap_or(Color::LightRed),
                        )
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Auto", "Short(5)", "Long(30)"])
                .foreground(
                    config
                        .style_color_symbol
                        .library_highlight()
                        .unwrap_or(Color::LightRed),
                )
                .rewind(true)
                .title(" Seek step in seconds: ", Alignment::Left)
                .value(seek_step),
            config: config.clone(),
        }
    }
}

impl Component<Msg, NoUserEvent> for ConfigSeekStep {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let config = self.config.clone();
        handle_radio_ev(
            self,
            ev,
            &config,
            Msg::ConfigEditor(ConfigEditorMsg::SeekStepBlurDown),
            Msg::ConfigEditor(ConfigEditorMsg::SeekStepBlurUp),
        )
    }
}
