use leptos::*;
use models::{Bookmark as BookmarkType, Id};

use crate::{
    api::use_entity,
    components::FlexSpace,
    icons::{PencilSquareIcon, XMarkIcon},
};

#[component]
pub fn Bookmark(
    cx: Scope,
    bookmark: BookmarkType,
    set_bookmark_form: WriteSignal<BookmarkType>,
    delete_bookmark: Action<Id<BookmarkType>, bool>,
) -> impl IntoView {
    // let id = store_value(cx, bookmark.id.clone());
    let bookmark = use_entity(cx, bookmark);

    view! { cx,
        <div class="flex w-full gap-1">
            <FlexSpace />
            <a
                class="text-2xl text-orange-200 hover:text-orange-400 underline"
                href=move || bookmark().url
            >
                { move || bookmark().name }
            </a>
            <FlexSpace />
            <button
                class="w-6 ml-2"
                on:click=move |_| set_bookmark_form(bookmark())
            >
                <PencilSquareIcon />
            </button>
            <button
                class="w-6"
                on:click=move |_| delete_bookmark.dispatch(bookmark().id)
            >
                <XMarkIcon />
            </button>
        </div>
    }
}
