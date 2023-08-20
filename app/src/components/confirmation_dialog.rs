use leptos::*;

use crate::components::Dialog;

#[component]
pub fn ConfirmationDialog<FYES, FNO>(cx: Scope, on_confirm: FYES, on_close: FNO) -> impl IntoView
where
    FYES: Fn() + Copy + 'static,
    FNO: Fn() + Copy + 'static,
{
    view! { cx,
        <Dialog on_close >
            "Are you sure you want to delete?"
            <div class="flex flex-row self-center gap-4">
                <button
                    class="bg-slate-600 w-fit rounded px-2 py-1 disabled:text-gray-400"
                    on:click=move |_| {
                        on_confirm();
                        on_close();
                    }
                >
                    "Yes"
                </button>
                <button
                    class="bg-slate-600 w-fit rounded px-2 py-1 disabled:text-gray-400"
                    on:click=move |_| on_close()
                >
                    "No"
                </button>
            </div>
        </Dialog>
    }
}
