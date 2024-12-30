use docx_rs::{self, DocumentChild};
use gpui::{
    div, img, prelude::*, rgb, px, App, AppContext, SharedString, 
    ViewContext, WindowOptions, IntoElement,
};

use std::fs;


struct Docx(docx_rs::Docx);

impl Docx {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read(file_path)?;
        let docx = docx_rs::read_docx(&file_content)?;
        Ok(Self(docx))
    }
}

impl Render for Docx {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            // .flex()
            // .flex_col()
            .bg(rgb(0xffffff))
            .size_full()
            .p_4()
            .children(self.0.document.children.iter().map(|child| {
                match child {
                    DocumentChild::Paragraph(para) => {
                        div()
                            .flex()
                            .children(para.children.iter().map(|p_child| {
                                match p_child {
                                    docx_rs::ParagraphChild::Run(run) => {
                                        let run_property = &run.run_property;
                                        div().flex().flex_wrap().children(
                                            run.children.iter().map(|r_child| match r_child {
                                                docx_rs::RunChild::Text(text) => {
                                                    div().flex().flex_wrap()
                                                        // .bg(run_property.color.clone())
                                                        // .flex()
                                                        // .flex_row()
                                                        // .text(text.clone())
                                                        // .text_size(run_property.size.clone())
                                                        // .text_weight(run_property.bold.clone())
                                                        // .text_style(run_property.italic.clone())
                                                        // .text_underline(run_property.underline.clone())
                                                        // .text_strike(run_property.strike.clone())
                                                        // .text_size(run_property.sz.clone())
                                                        .text_color(rgb(0x000000))
                                                        .child(text.text.clone())
                                                        .into_any_element()
                                                }
                                                _ => {
                                                    div().flex().flex_wrap()
                                                        // .flex()
                                                        // .flex_row()
                                                        .text_color(rgb(0x000000))
                                                        .child("Image placeholder")
                                                        .into_any_element()
                                                }
                                            })
                                        ).into_any_element()
                                    }
                                    _ => div().into_any_element()
                                }
                        })).into_any_element()
                    }
                    _ => div().into_any_element()
                }
            }))
    }
}

pub trait RenderUI: 'static + Sized {
    fn render(&mut self) -> impl IntoElement;
}

struct Paragraph(docx_rs::Paragraph);

impl RenderUI for Paragraph {
    fn render(&mut self) -> impl IntoElement {
        div()
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            // 替换为实际的 DOCX 文件路径
            let docx_preview = Docx::new("sample.docx").unwrap();
            
            cx.new_view(|_cx| docx_preview)
        })
        .unwrap();
    });
}