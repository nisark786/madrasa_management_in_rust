use leptos::*;

#[component]
pub fn Button(
    #[prop(into, default = "button".to_string())] variant: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(into, default = "w-full".to_string())] class: String,
    children: Children,
    #[prop(optional)] on_click: Option<impl Fn(leptos::ev::MouseEvent) + 'static>,
) -> impl IntoView {
    let base_class = match variant.as_str() {
        "primary" => "bg-gradient-to-r from-blue-600 to-blue-700 text-white hover:from-blue-700 hover:to-blue-800 shadow-md hover:shadow-lg",
        "secondary" => "bg-slate-100 text-slate-900 hover:bg-slate-200 border border-slate-200",
        "outline" => "border-2 border-blue-600 text-blue-600 hover:bg-blue-50",
        "danger" => "bg-red-600 text-white hover:bg-red-700",
        _ => "bg-slate-100 text-slate-900 hover:bg-slate-200",
    };

    let disabled_class = if disabled || loading {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    };

    view! {
        <button
            class=format!(
                "px-6 py-2.5 rounded-lg font-500 text-sm transition-all duration-200 ease-in-out {} {} {}",
                base_class,
                disabled_class,
                class
            )
            disabled=disabled || loading
            on:click=move |e| {
                if !disabled && !loading {
                    if let Some(handler) = &on_click {
                        handler(e)
                    }
                }
            }
        >
            {if loading {
                view! { <span class="inline-block animate-spin mr-2">"⟳"</span> }.into_view()
            } else {
                "".into_view()
            }}
            {children()}
        </button>
    }
}

#[component]
pub fn Card(children: Children, #[prop(into, default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <div class=format!(
            "bg-white rounded-xl shadow-sm border border-slate-100 p-6 hover:shadow-md transition-shadow duration-200 {}",
            class
        )>
            {children()}
        </div>
    }
}

#[component]
pub fn Input(
    #[prop(into, default = "text".to_string())] input_type: String,
    #[prop(into, default = "".to_string())] placeholder: String,
    #[prop(optional)] value: Option<String>,
    #[prop(optional)] on_input: Option<impl Fn(String) + 'static>,
    #[prop(default = false)] required: bool,
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <input
            type=input_type
            placeholder=placeholder
            value=value.unwrap_or_default()
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    handler(event_target_value(&ev))
                }
            }
            required=required
            class=format!(
                "w-full px-4 py-2.5 bg-white border border-slate-200 rounded-lg focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-100 transition-all duration-200 text-sm {}",
                class
            )
        />
    }
}

#[component]
pub fn Label(#[prop(optional, into)] for_id: Option<String>, children: Children) -> impl IntoView {
    view! {
        <label
            for=for_id
            class="block text-sm font-500 text-slate-700 mb-2"
        >
            {children()}
        </label>
    }
}

#[component]
pub fn FormGroup(children: Children) -> impl IntoView {
    view! {
        <div class="mb-5">
            {children()}
        </div>
    }
}

#[component]
pub fn Badge(
    #[prop(into, default = "default".to_string())] variant: String,
    children: Children,
) -> impl IntoView {
    let (bg_class, text_class) = match variant.as_str() {
        "success" => ("bg-green-100", "text-green-700"),
        "warning" => ("bg-amber-100", "text-amber-700"),
        "danger" => ("bg-red-100", "text-red-700"),
        "info" => ("bg-blue-100", "text-blue-700"),
        _ => ("bg-slate-100", "text-slate-700"),
    };

    view! {
        <span class=format!(
            "inline-flex items-center px-3 py-1 rounded-full text-xs font-600 {} {}",
            bg_class,
            text_class
        )>
            {children()}
        </span>
    }
}

#[component]
pub fn Alert(
    #[prop(into, default = "info".to_string())] variant: String,
    children: Children,
) -> impl IntoView {
    let (bg_class, border_class, text_class) = match variant.as_str() {
        "success" => ("bg-green-50", "border-green-200", "text-green-800"),
        "error" => ("bg-red-50", "border-red-200", "text-red-800"),
        "warning" => ("bg-amber-50", "border-amber-200", "text-amber-800"),
        _ => ("bg-blue-50", "border-blue-200", "text-blue-800"),
    };

    view! {
        <div class=format!(
            "p-4 rounded-lg border {} {} text-sm",
            bg_class,
            border_class
        )>
            <p class=text_class>{children()}</p>
        </div>
    }
}

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center h-screen">
            <div class="text-center">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                <p class="text-slate-600">"Loading..."</p>
            </div>
        </div>
    }
}
