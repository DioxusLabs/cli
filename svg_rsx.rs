
fn component(cx: Scope) -> Element {
    cx.render(rsx!(
        div { 
            icons::icon_0 {}
            icons::icon_1 {}
            icons::icon_2 {}
            icons::icon_3 {}
        }
    ))
}

mod icons {
	use super::*;
    pub(super) fn icon_0(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "h-5 w-5 text-gray-500",
                fill: "currentColor",
                view_box: "0 0 20 20",
                path { 
                    d: "M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z",
                    clip_rule: "evenodd",
                    fill_rule: "evenodd",
                }
            }
		))
	}
    pub(super) fn icon_1(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "h-5 w-5 text-gray-500",
                view_box: "0 0 20 20",
                fill: "currentColor",
                path { 
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z",
                }
            }
		))
	}
    pub(super) fn icon_2(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "h-5 w-5 text-gray-500",
                fill: "currentColor",
                view_box: "0 0 20 20",
                path { 
                    d: "M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z",
                    clip_rule: "evenodd",
                    fill_rule: "evenodd",
                }
            }
		))
	}
    pub(super) fn icon_3(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "h-5 w-5 text-gray-500",
                fill: "currentColor",
                view_box: "0 0 20 20",
                path { 
                    fill_rule: "evenodd",
                    d: "M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z",
                    clip_rule: "evenodd",
                }
            }
		))
	}
}
