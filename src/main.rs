// use gpui::{div, prelude::*, rgb, App, AppContext, SharedString, ViewContext, WindowOptions, FontWeight, Rgb};
use gpui::*;
// 定义文档基本元素的数据结构
#[derive(Debug, Clone)]
enum TextStyle {
    Bold,
    Italic,
    Regular,
}

#[derive(Debug, Clone)]
struct TextSpan {
    content: SharedString,
    style: TextStyle,
    font_size: f32,
    font_family: SharedString,
    color: Rgba,
    background_color: Option<Rgba>,
}

#[derive(Debug, Clone)]
struct Paragraph {
    spans: Vec<TextSpan>,
    alignment: TextAlignment,
}

#[derive(Debug, Clone)]
enum TextAlignment {
    Left,
    Center,
    Right,
}

struct DocxDocument {
    paragraphs: Vec<Paragraph>,
}

// 修改 HelloWorld 为 DocxViewer
struct DocxViewer {
    document: DocxDocument,
}

impl Render for DocxViewer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        // 先克隆数据，这样就不会有生命周期问题
        let paragraphs = self.document.paragraphs.clone();
        
        div()
            .flex()
            .flex_col()
            .w_128()
            .size( AbsoluteLength::Rems(Rems(12.0)))
            .bg(rgb(0xffffff))
            // .size_full()
            .p_4()
            .children(paragraphs.into_iter().map(|para| {
                div()
                    .flex()
                    // .gap_1()
                    .shadow_sm()
                    .children(para.spans.into_iter().map(|span| {
                        let mut style = match span.style {
                            TextStyle::Bold => div().font_weight(FontWeight::BOLD),
                            TextStyle::Italic => div().italic(),
                            TextStyle::Regular => div(),
                        };
                        
                        // 应用额外的样式
                        style = style

                            // .text_size(span.font_size)
                            .font_family(span.font_family.to_string())
                            .text_color(span.color);
                        
                        // 如果有背景色，则应用背景色
                        if let Some(bg_color) = span.background_color {
                            style = style.bg(bg_color);
                        }
                        
                        style.child(span.content.to_string())
                        
                    }))
            }))
            .child(div().text_xl().w_full().child("After completely rewriting Zed's UI on top of the new framework, we've spent the last 2-3 weeks using revamped version of Zed as our daily driver. Now you can try it out as well. Most of the benefits of this preview release are internal, and will be revealed over time. We have however added one popular feature request: The ability to customize and scale the UI font.".to_string()))
    }
}

fn main() {
    // 创建示例文档
    let document = DocxDocument {
        paragraphs: vec![
            // Paragraph {
                // spans: vec![
                    // TextSpan {
                        // content: "这是一个".into(),
                        // style: TextStyle::Regular,
                        // font_size: 16.0,
                        // font_family: "Arial".into(),
                        // color: rgb(0x000000),
                        // background_color: None,
                    // },
                    // TextSpan {
                        // content: "示例".into(),
                        // style: TextStyle::Bold,
                        // font_size: 18.0,
                        // font_family: "Microsoft YaHei".into(),
                        // color: rgb(0xFF0000),
                        // background_color: Some(rgb(0xFFFF00)),
                    // },
                    // TextSpan {
                        // content: "文档".into(),
                        // style: TextStyle::Italic,
                        // font_size: 14.0,
                        // font_family: "SimSun".into(),
                        // color: rgb(0x0000FF),
                        // background_color: None,
                    // },
                // ],
                // alignment: TextAlignment::Left,
            // },
            Paragraph {
                spans: vec![
                    TextSpan {
                        content: "After completely rewriting Zed's UI on top of the new framework, we've spent the last 2-3 weeks using revamped version of Zed as our daily driver. Now you can try it out as well. Most of the benefits of this preview release are internal, and will be revealed over time. We have however added one popular feature request: The ability to customize and scale the UI font.".into(),
                        style: TextStyle::Regular,
                        font_size: 16.0,
                        font_family: "Arial".into(),
                        color: rgb(0x000000),
                        background_color: None,
                    },
                    TextSpan {
                        content: "示例".into(),
                        style: TextStyle::Bold,
                        font_size: 18.0,
                        font_family: "Microsoft YaHei".into(),
                        color: rgb(0xFF0000),
                        background_color: Some(rgb(0xFFFF00)),
                    },
                    TextSpan {
                        content: "文档".into(),
                        style: TextStyle::Italic,
                        font_size: 14.0,
                        font_family: "SimSun".into(),
                        color: rgb(0x0000FF),
                        background_color: None,
                    },
                ],
                alignment: TextAlignment::Left,
            },
                        
        ],
    };

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| DocxViewer { document })
        })
        .unwrap();
    });
}