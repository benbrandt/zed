use gpui::{div, Div, ParentElement, Render, Styled, View, VisualContext, WindowContext};
use indoc::indoc;
use story::*;

pub struct TextStory;

impl TextStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|_cx| Self)
    }
}

impl Render for TextStory {
    type Element = Div;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        // let # = "The quick brown fox jumps over the lazy dog. Meanwhile, the lazy dog decided it was time for a change. He started daily workout routines, ate healthier and became the fastest dog in town.";

        Story::container2::<TextStory>("crates/storybook2/src/stories/text.rs").child(
            StorySection::new().child(
                StoryItem::new(
                    "Default Text",
                    div().flex().child(div().max_w_96().child("foo")),
                )
                .description("Text with a max-width. Wraps based on set max-width.")
                .usage(indoc! {r##"
                    div().max_w_96()
                        .child("Some text that you want to wrap.")
                    "##
                }),
            ),
        )
    }
}

// impl Render for TextStory {
//     type Element = Div;

//     fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
//         v_stack()
//             .bg(blue())
//             .child(
//                 div()
//                     .flex()
//                     .child(div().max_w_96().bg(white()).child(concat!(
//         "max-width: 96. The quick brown fox jumps over the lazy dog. ",
//         "Meanwhile, the lazy dog decided it was time for a change. ",
//         "He started daily workout routines, ate healthier and became the fastest dog in town.",
//     ))),
//             )
//             .child(div().h_5())
//             .child(div().flex().flex_col().w_96().bg(white()).child(concat!(
//         "flex-col. width: 96; The quick brown fox jumps over the lazy dog. ",
//         "Meanwhile, the lazy dog decided it was time for a change. ",
//         "He started daily workout routines, ate healthier and became the fastest dog in town.",
//     )))
//             .child(div().h_5())
//             .child(
//                 div()
//                     .flex()
//                     .child(div().min_w_96().bg(white()).child(concat!(
//     "min-width: 96. The quick brown fox jumps over the lazy dog. ",
//     "Meanwhile, the lazy dog decided it was time for a change. ",
//     "He started daily workout routines, ate healthier and became the fastest dog in town.",
// ))))
//             .child(div().h_5())
//             .child(div().flex().w_96().bg(white()).child(div().overflow_hidden().child(concat!(
//         "flex-row. width 96. overflow-hidden. The quick brown fox jumps over the lazy dog. ",
//         "Meanwhile, the lazy dog decided it was time for a change. ",
//         "He started daily workout routines, ate healthier and became the fastest dog in town.",
//     ))))
//             // NOTE: When rendering text in a horizonal flex container,
//             // Taffy will not pass width constraints down from the parent.
//             // To fix this, render text in a parent with overflow: hidden
//                     .child(div().h_5())
//                     .child(div().flex().w_96().bg(red()).child(concat!(
//                 "flex-row. width 96. The quick brown fox jumps over the lazy dog. ",
//                 "Meanwhile, the lazy dog decided it was time for a change. ",
//                 "He started daily workout routines, ate healthier and became the fastest dog in town.",
//             ))).child(
//                 InteractiveText::new(
//                     "interactive",
//                     StyledText::new("Hello world, how is it going?").with_highlights(&cx.text_style(), [
//                         (6..11, HighlightStyle {
//                             background_color: Some(green()),
//                             ..Default::default()
//                         }),
//                     ]),
//                 )
//                 .on_click(vec![2..4, 1..3, 7..9], |range_ix, _cx| {
//                     println!("Clicked range {range_ix}");
//                 })
//             )
//     }
// }
