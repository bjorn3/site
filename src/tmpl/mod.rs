use crate::app::Config;
use maud::{html, Markup};
use std::sync::Arc;

pub mod nag;

pub fn xeblog_salary_history(cfg: Arc<Config>) -> Markup {
    html! {
        table.salary_history {
            tr {
                th { "Title" }
                th { "Start Date" }
                th { "End Date" }
                th { "Days Worked" }
                th { "Salary" }
                th { "How I Left" }
            }
            @for job in &cfg.clone().job_history {
                (job.pay_history_row())
            }
        }
    }
}

pub fn xeblog_hero(file: String, prompt: Option<String>) -> Markup {
    html! {
        figure.hero style="margin:0" {
            picture style="margin:0" {
                source type="image/avif" srcset={"https://cdn.xeiaso.net/file/christine-static/hero/" (file) ".avif"};
                source type="image/webp" srcset={"https://cdn.xeiaso.net/file/christine-static/hero/" (file) ".webp"};
                img style="padding:0" alt={"hero image " (file)} src={"https://cdn.xeiaso.net/file/christine-static/hero/" (file) "-smol.png"};
            }
            figcaption { "Image generated by MidJourney" @if let Some(prompt) = prompt { " -- " (prompt) } }
        }
    }
}

pub fn xeblog_conv(name: String, mood: String, body: Markup) -> Markup {
    let name_lower = name.clone().to_lowercase();

    html! {
        .conversation {
            ."conversation-picture"."conversation-smol" {
                picture {
                    source type="image/avif" srcset={"https://cdn.xeiaso.net/file/christine-static/stickers/" (name_lower) "/" (mood) ".avif"};
                    source type="image/webp" srcset={"https://cdn.xeiaso.net/file/christine-static/stickers/" (name_lower) "/" (mood) ".webp"};
                    img alt={(name) " is " (mood)} src={"https://cdn.xeiaso.net/file/christine-static/stickers/" (name_lower) "/" (mood) ".png"};
                }
            }
            ."conversation-chat" {
                "<"
                b { (name) }
                "> "
                (body)
            }
        }
    }
}
