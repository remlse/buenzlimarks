use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

use super::DbTrait;

pub fn insert_seeds(db: &(dyn DbTrait + Send + Sync)) {
    // user(id), pages, widgets, bookmarks(name, url)
    #[allow(clippy::type_complexity)]
    let data: &[(Id<User>, &[&[&[(&str, &str)]]])] = &[(
        crate::models::user::dev_user_id(),
        &[&[
            &[
                (
                    "Requirements",
                    "https://github.com/users/remlse/projects/1/views/6",
                ),
                (
                    "Prioritization",
                    "https://github.com/users/remlse/projects/1/views/7",
                ),
                (
                    "Tasks",
                    "https://github.com/users/remlse/projects/1/views/2",
                ),
            ],
            &[
                ("YouTube", "https://youtube.com"),
                ("Rust std docs", "https://std.rs"),
            ],
        ]],
    )];

    for user in data {
        let user_id = user.0.clone();
        for page in user.1 {
            let p_id = Id::random();
            db.insert_page(&user_id, &Page { id: p_id.clone() })
                .unwrap();
            for widget in page.iter() {
                let w_id = Id::random();
                db.insert_widget(
                    &user_id,
                    &Widget {
                        id: w_id.clone(),
                        page_id: p_id.clone(),
                    },
                )
                .unwrap();
                for (name, url) in widget.iter().copied() {
                    let bm_id = Id::random();
                    let bookmark = Bookmark {
                        id: bm_id,
                        name: name.into(),
                        url: url.into(),
                        widget_id: w_id.clone(),
                    };
                    db.insert_bookmark(&user_id, bookmark).unwrap();
                }
            }
        }
    }
}
