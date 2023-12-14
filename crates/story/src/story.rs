use gpui::{div, hsla, AnyElement, Div, ElementId, Hsla, SharedString, Stateful, WindowContext};
use gpui::{prelude::*, px};
use smallvec::SmallVec;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn reasonably_unique_id() -> String {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap();

    let cnt = COUNTER.fetch_add(1, Ordering::Relaxed);

    let id = format!("{}_{}", timestamp.as_nanos(), cnt);

    id
}

pub struct StoryColor {
    pub primary: Hsla,
    pub secondary: Hsla,
    pub border: Hsla,
    pub background: Hsla,
    pub card_background: Hsla,
    pub divider: Hsla,
    pub link: Hsla,
}

impl StoryColor {
    pub fn new() -> Self {
        Self {
            primary: hsla(216. / 360., 11. / 100., 0. / 100., 1.),
            secondary: hsla(216. / 360., 11. / 100., 16. / 100., 1.),
            border: hsla(216. / 360., 11. / 100., 91. / 100., 1.),
            background: hsla(0. / 360., 0. / 100., 100. / 100., 1.),
            card_background: hsla(0. / 360., 0. / 100., 96. / 100., 1.),
            divider: hsla(216. / 360., 11. / 100., 91. / 100., 1.),
            link: hsla(206. / 360., 100. / 100., 50. / 100., 1.),
        }
    }
}

pub fn story_color() -> StoryColor {
    StoryColor::new()
}

pub struct Story {}

impl Story {
    pub fn container() -> Div {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(story_color().background)
    }

    // TODO: Move all stories to container2, then rename
    pub fn container2<T>(relative_path: &'static str) -> Div {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(story_color().background)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .p_2()
                    .border_b()
                    .border_color(story_color().border)
                    .child(Story::title_for::<T>())
                    .child(
                        div()
                            .text_xs()
                            .text_color(story_color().primary)
                            .child(Story::open_story_link(relative_path)),
                    ),
            )
    }

    pub fn open_story_link(relative_path: &'static str) -> impl Element {
        let path = PathBuf::from_iter([relative_path]);
        div()
            .id(SharedString::from(format!("id_{}", relative_path)))
            .text_xs()
            .text_color(story_color().primary)
            .on_click({
                let path = path.clone();

                move |_event, _cx| {
                    let path = format!("{}:0:0", path.to_string_lossy());

                    std::process::Command::new("zed").arg(path).spawn().ok();
                }
            })
            .child(Story::link(path.to_string_lossy().to_string()))
    }

    pub fn title(title: impl Into<SharedString>) -> impl Element {
        div()
            .text_xs()
            .text_color(story_color().primary)
            .child(title.into())
    }

    pub fn title_for<T>() -> impl Element {
        Self::title(std::any::type_name::<T>())
    }

    pub fn section() -> Div {
        div().mt_4().mb_2()
    }

    pub fn section_title() -> Div {
        div().text_lg().text_color(story_color().primary)
    }

    pub fn group() -> Div {
        div().my_2().bg(story_color().background)
    }

    pub fn code_block(code: impl Into<SharedString>) -> Div {
        div()
            .size_full()
            .p_2()
            .bg(gpui::black())
            .border()
            .border_color(story_color().border)
            .rounded_md()
            .text_sm()
            .text_color(gpui::white())
            .child(code.into())
    }

    pub fn divider() -> Div {
        div().my_2().h(px(1.)).bg(story_color().divider)
    }

    pub fn link(link: impl Into<SharedString>) -> impl Element {
        div()
            .id(ElementId::from(SharedString::from(reasonably_unique_id())))
            .text_xs()
            .text_color(story_color().link)
            .cursor(gpui::CursorStyle::PointingHand)
            .child(link.into())
    }

    pub fn description(description: impl Into<SharedString>) -> impl Element {
        div()
            .text_sm()
            .text_color(story_color().secondary)
            .min_w_96()
            .child(description.into())
    }

    pub fn label(label: impl Into<SharedString>) -> impl Element {
        div()
            .text_xs()
            .text_color(story_color().primary)
            .child(label.into())
    }

    /// Note: Not ui::v_stack() as the story crate doesn't depend on the ui crate.
    pub fn v_stack() -> Div {
        div().flex().flex_col().gap_1()
    }
}

#[derive(IntoElement)]
pub struct StoryItem {
    label: SharedString,
    item: AnyElement,
    description: Option<SharedString>,
    usage: Option<SharedString>,
}

impl StoryItem {
    pub fn new(label: impl Into<SharedString>, item: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            item: item.into_any_element(),
            description: None,
            usage: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn usage(mut self, code: impl Into<SharedString>) -> Self {
        self.usage = Some(code.into());
        self
    }
}

impl RenderOnce for StoryItem {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        div()
            .my_2()
            .flex()
            .w_full()
            .child(
                Story::v_stack()
                    .px_2()
                    .flex_none()
                    .w_1_2()
                    .min_h_px()
                    .child(Story::label(self.label))
                    .child(
                        div()
                            .rounded_sm()
                            .bg(story_color().card_background)
                            .border()
                            .border_color(story_color().border)
                            .child(self.item),
                    )
                    .when_some(self.description, |this, description| {
                        this.child(Story::description(description))
                    }),
            )
            .child(
                Story::v_stack()
                    .px_2()
                    .flex_none()
                    .w_1_2()
                    .min_h_px()
                    .when_some(self.usage, |this, usage| {
                        this.child(Story::label("Usage"))
                            .child(Story::code_block(usage))
                    }),
            )
    }
}

#[derive(IntoElement)]
pub struct StorySection {
    description: Option<SharedString>,
    children: SmallVec<[AnyElement; 2]>,
}

impl StorySection {
    pub fn new() -> Self {
        Self {
            description: None,
            children: SmallVec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl RenderOnce for StorySection {
    type Rendered = Div;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        Story::section()
            // Section title
            .py_2()
            // Section description
            .when_some(self.description.clone(), |section, description| {
                section.child(Story::description(description))
            })
            .child(div().flex().flex_col().gap_2().children(self.children))
            .child(Story::divider())
    }
}

impl ParentElement for StorySection {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}
