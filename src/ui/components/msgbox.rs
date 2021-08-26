//! ## MsgBox
//!
//! `MsgBox` component renders a simple readonly no event associated centered text

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
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
// deps
// locals
// use crate::ui::align_text_center;
// ext
use tui_realm_stdlib::utils::get_block;
// use tui_realm_stdlib::utils::{get_block, use_or_default_styles};
use tuirealm::event::Event;
use tuirealm::props::{
    Alignment, BlockTitle, BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextSpan,
};
use tuirealm::tui::{
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    // text::{Span, Spans},
    widgets::{BorderType, Borders, List, ListItem},
};
use tuirealm::{Component, Frame, Msg, Payload};

// -- Props

const PROP_TITLE: &str = "title";
const PROP_SPANS: &str = "spans";

pub struct MsgBoxPropsBuilder {
    props: Option<Props>,
}

impl Default for MsgBoxPropsBuilder {
    fn default() -> Self {
        MsgBoxPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for MsgBoxPropsBuilder {
    fn build(&mut self) -> Props {
        let mut props: Props = Props::default();
        if let Some(p) = self.props.take() {
            props = p;
        }
        props
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for MsgBoxPropsBuilder {
    fn from(props: Props) -> Self {
        MsgBoxPropsBuilder { props: Some(props) }
    }
}

impl MsgBoxPropsBuilder {
    #[allow(dead_code)]
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    #[allow(dead_code)]
    pub fn bold(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::BOLD;
        }
        self
    }

    #[allow(dead_code)]
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    #[allow(dead_code)]
    pub fn with_texts(&mut self, title: Option<String>, spans: Vec<TextSpan>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            // props.own = TextParts::new(title, Some(texts));
            props.own.insert(
                PROP_TITLE,
                PropPayload::One(PropValue::Str(
                    title.as_ref().unwrap_or(&"".to_string()).to_string(),
                )),
            );
            props.own.insert(
                PROP_SPANS,
                PropPayload::Vec(spans.into_iter().map(PropValue::TextSpan).collect()),
            );
        }
        self
    }

    #[allow(dead_code)]
    pub fn with_title<S: AsRef<str>>(&mut self, text: S, alignment: Alignment) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.title = Some(BlockTitle::new(text, alignment));
        }
        self
    }
}

// -- component

pub struct MsgBox {
    props: Props,
}

impl MsgBox {
    /// ### new
    ///
    /// Instantiate a new Text component
    #[allow(dead_code)]
    pub fn new(props: Props) -> Self {
        MsgBox { props }
    }
}

impl Component for MsgBox {
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.visible {
            let lines: Vec<ListItem> = match self.props.own.get(PROP_SPANS).as_ref() {
                Some(PropPayload::Vec(spans)) => spans
                    .iter()
                    .map(|x| x.unwrap_text_span())
                    .map(|x| {
                        tui_realm_stdlib::utils::wrap_spans(
                            vec![x.clone()].as_slice(),
                            area.width as usize - 2,
                            &self.props,
                        )
                    })
                    .map(ListItem::new)
                    .collect(),
                _ => Vec::new(),
            };
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            render.render_widget(
                List::new(lines)
                    .block(get_block(
                        &self.props.borders,
                        Some(&BlockTitle::new(title.unwrap_or(""), Alignment::Left)),
                        true,
                    ))
                    .start_corner(Corner::TopLeft)
                    .style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background),
                    ),
                area,
            );
        }
    }

    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // Return None
        Msg::None
    }

    fn get_props(&self) -> Props {
        self.props.clone()
    }

    fn on(&mut self, ev: Event) -> Msg {
        // Return key
        if let Event::Key(key) = ev {
            Msg::OnKey(key)
        } else {
            Msg::None
        }
    }

    fn get_state(&self) -> Payload {
        Payload::None
    }

    fn blur(&mut self) {}

    fn active(&mut self) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::event::{KeyCode, KeyEvent};
    use tuirealm::props::TextSpan;
    use tuirealm::tui::style::Color;

    #[test]
    fn test_ui_components_msgbox() {
        let mut component: MsgBox = MsgBox::new(
            MsgBoxPropsBuilder::default()
                .hidden()
                .visible()
                .with_foreground(Color::Red)
                .bold()
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_texts(
                    None,
                    vec![
                        TextSpan::from("Press "),
                        TextSpan::new("<ESC>").fg(Color::Cyan).bold(),
                        TextSpan::from(" to quit"),
                    ],
                )
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert!(component.props.modifiers.intersects(Modifier::BOLD));
        assert_eq!(component.props.visible, true);
        // assert_eq!(component.props.own.get(PROP_SPANS).unwrap(),
        //     &PropPayload::Vec(vec![
        //         PropValue::TextSpan(TextSpan::from("welcome to ")),
        //         PropValue::TextSpan(TextSpan::from("tui-realm")),
        //     ])

        // );
        component.active();
        component.blur();
        // Update
        let props = MsgBoxPropsBuilder::from(component.get_props())
            .hidden()
            .with_foreground(Color::Yellow)
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.visible, false);
        assert_eq!(component.props.foreground, Color::Yellow);
        // Get value
        assert_eq!(component.get_state(), Payload::None);
        // Event
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
    }
}
