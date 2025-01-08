use leptos::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use leptos::prelude::*;
use wasm_bindgen_futures;

use charming::{
    component::*,
    element::*,
    element::{
        Emphasis, ItemStyle, Label, LabelLine, LabelPosition, LineStyle, LineStyleType, Sort,
        Tooltip, Trigger,
    },
    datatype::DataPointItem,
    series::*,
    Chart, WasmRenderer,
    df
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-ssr-test-charming.css"/>

        // sets the document title
        <Title text="Welcome to Leptos-Charming"/>

        // Add ECharts script
        <script src="https://cdnjs.cloudflare.com/ajax/libs/echarts/5.4.3/echarts.min.js"></script>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (mon, set_mon) = signal(150);
    let (tue, set_tue) = signal(230);
    let (wed, set_wed) = signal(224);
    let (thur, set_thur) = signal(218);
    let (fri, set_fri) = signal(135);
    let (sat, set_sat) = signal(147);
    let (sun, set_sun) = signal(260);
    let (typenum, set_typenum) = signal(0);
    #[cfg(feature = "ssr")]
    let action =  Action::new(|_| async{});
    #[cfg(not(feature = "ssr"))]
    let action =  Action::new(move |_input: &() | {
    // Capture current values at the time of action
    let current_mon = mon.get_untracked();
    let current_tue = tue.get_untracked();
    let current_wed = wed.get_untracked();
    let current_thur = thur.get_untracked();
    let current_fri = fri.get_untracked();
    let current_sat = sat.get_untracked();
    let current_sun = sun.get_untracked();
    let current_type = typenum.get_untracked();

     async move {
        // Initialize renderer with smaller dimensions to reduce memory usage
        let renderer = WasmRenderer::new(1000, 700);
        let chart = match current_type {
            0 => {
                Chart::new()
                    .title(Title::new().text("Demo: Leptos + Charming"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Line::new().data(vec![
                        current_mon,
                        current_tue,
                        current_wed,
                        current_thur,
                        current_fri,
                        current_sat,
                        current_sun
                    ]))
            }
            1 => {
                // Simple Bar Chart
                Chart::new()
                    .title(Title::new().text("Simple Bar Chart"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Bar::new().data(vec![
                        current_mon,
                        current_tue,
                        current_wed,
                        current_thur,
                        current_fri,
                        current_sat,
                        current_sun
                    ]))
            }
            2 => {
                // Bar Chart with Custom Style for a Data Point
                Chart::new()
                    .title(Title::new().text("Bar Chart with Highlighted Point"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(Bar::new().data(vec![
                        current_mon.into(),
                        DataPointItem::new(current_tue).item_style(ItemStyle::new().color("#a90000")),
                        current_wed.into(),
                        current_thur.into(),
                        current_fri.into(),
                        current_sat.into(),
                        current_sun.into(),
                    ]))
            }
            3 => {
                // Bar Chart with Background Style
                Chart::new()
                    .title(Title::new().text("Bar Chart with Background"))
                    .x_axis(
                        Axis::new()
                            .type_(AxisType::Category)
                            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                    )
                    .y_axis(Axis::new().type_(AxisType::Value))
                    .series(
                        Bar::new()
                            .show_background(true)
                            .background_style(BackgroundStyle::new().color("rgba(180, 180, 180, 0.2)"))
                            .data(vec![
                                current_mon,
                                current_tue,
                                current_wed,
                                current_thur,
                                current_fri,
                                current_sat,
                                current_sun
                            ]),
                    )
            }
            4 => {Chart::new()
            .legend(Legend::new().top("bottom"))
            .series(
                Pie::new()
                    .name("Nightingale Chart")
                    .rose_type(PieRoseType::Radius)
                    .radius(vec!["50", "250"])
                    .center(vec!["50%", "50%"])
                    .item_style(ItemStyle::new().border_radius(8))
                    .data(vec![
                        (current_mon, "Monday"),
                        (current_tue, "Tuesday"),
                        (current_wed, "Wednesday"),
                        (current_thur, "Thursday"),
                        (current_fri, "Friday"),
                        (current_sat, "Saturday"),
                        (current_sun, "Sunday"),
                    ])
            )} // Default empty chart
            _ => {
                Chart::new()
                .title(Title::new().text("Funnel"))
                .tooltip(
                    Tooltip::new()
                        .trigger(Trigger::Item)
                        .formatter("{a} <br/>{b} : {c}%"),
                )
                .toolbox(
                    Toolbox::new().feature(
                        Feature::new()
                            .data_view(DataView::new().read_only(false))
                            .restore(Restore::new())
                            .save_as_image(SaveAsImage::new()),
                    ),
                )
                .legend(Legend::new().data(vec!["Show", "Click", "Visit", "Inquiry", "Order"]))
                .series(
                    Funnel::new()
                        .name("Funnel")
                        .left("10%")
                        .top(60)
                        .bottom(60)
                        .width("80%")
                        .min(0)
                        .max(100)
                        .min_size("0%")
                        .max_size("100%")
                        .sort(Sort::Descending)
                        .gap(2)
                        .label(Label::new().show(true).position(LabelPosition::Inside))
                        .label_line(
                            LabelLine::new()
                                .length(10)
                                .line_style(LineStyle::new().width(1).type_(LineStyleType::Solid)),
                        )
                        .item_style(ItemStyle::new().border_color("#fff").border_width(1))
                        .emphasis(Emphasis::new().label(Label::new().font_size(20)))
                        .data(df![
                            (current_mon, "Monday"),
                            (current_tue, "Tuesday"),
                            (current_wed, "Wednesday"),
                            (current_thur, "Thursday"),
                            (current_fri, "Friday"),
                            (current_sat, "Saturday"),
                            (current_sun, "Sunday"),
                        ]),
                )
            }
        };

        wasm_bindgen_futures::spawn_local(async move {
            match renderer.render("chart", &chart) {
                Ok(_) => log::info!("Chart rendered successfully"),
                Err(e) => log::error!("Failed to render chart: {:?}", e),
            }
        });
    }
});
    // Add this line to trigger initial render
    action.dispatch(());

    Effect::new(move |_| {
        // This will run whenever any of the signals change
        if mon.get() > 0 || tue.get() > 0 || wed.get() > 0 || 
           thur.get() > 0 || fri.get() > 0 || sat.get() > 0 || 
           sun.get() > 0 || typenum.get() >= 0 {
            action.dispatch(());
        }
    });
    view! { 
        <div class="metric">
            <div class="chart-field">
                <div id="chart"></div>
                <div class="setting-field">
                    <div class="dataset-field">
                        <input type="number"
                        on:input:target=move |ev| {
                            set_mon.set(ev.target().value().parse().unwrap());
                        }
                        prop:value=mon
                        class="num-input"
                        />
                        <p>"mon is: " {mon}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_tue.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=tue
                            class="num-input"
                        />
                        <p>"tue is: " {tue}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_wed.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=wed
                            class="num-input"
                        />
                        <p>"wed is: " {wed}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_thur.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=thur
                            class="num-input"
                        />
                        <p>"thur is: " {thur}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_fri.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=fri
                            class="num-input"
                        />
                        <p>"fri is: " {fri}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_sat.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=sat
                            class="num-input"
                        />
                        <p>"sat is: " {sat}</p>
                        <input type="number"
                            on:input:target=move |ev| {
                                set_sun.set(ev.target().value().parse().unwrap());
                            }
                            prop:value=sun
                            class="num-input"
                        />
                        <p>"sun is: " {sun}</p>
                    </div>
                    <select
                        class="type-setting"
                        on:change:target=move |ev| {
                        set_typenum.set(ev.target().value().parse().unwrap());
                        }
                        prop:value=move || typenum.get().to_string()
                    >
                        <option value="0">"Line"</option>
                        <option value="1">"Bar1"</option>
                        <option value="2">"Bar2"</option>
                        <option value="3">"Bar3"</option>
                        <option value="4">"Piechart"</option>
                        <option value="5">"Funnel"</option>
                    </select>
                </div>
            </div>
            <button on:click=move |_| {action.dispatch(());} class="draw-btn">"Show chart"</button>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
