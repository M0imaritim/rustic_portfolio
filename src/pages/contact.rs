use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (submitted, set_submitted) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let n = name.get();
        let e = email.get();
        let m = message.get();

        if n.is_empty() || e.is_empty() || m.is_empty() {
            set_error.set(Some("Please fill in all fields.".to_string()));
            return;
        }
        if !e.contains('@') {
            set_error.set(Some("Please enter a valid email.".to_string()));
            return;
        }

        set_error.set(None);
        set_loading.set(true);

        let body = format!(
            r#"{{"name":"{}","email":"{}","message":"{}"}}"#,
            n, e, m
        );

        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsValue;
            use wasm_bindgen_futures::spawn_local;
            use web_sys::{Request, RequestInit, RequestMode, Response};

            spawn_local(async move {
                let mut opts = RequestInit::new();
                opts.set_method("POST");
                opts.set_mode(RequestMode::Cors);
                opts.set_body(&JsValue::from_str(&body));

                let request = Request::new_with_str_and_init(
                    "https://formspree.io/f/YOUR_FORM_ID",
                    &opts,
                ).unwrap();

                request.headers().set("Content-Type", "application/json").unwrap();
                request.headers().set("Accept", "application/json").unwrap();

                let window = web_sys::window().unwrap();
                let resp: Result<JsValue, JsValue> =
                    wasm_bindgen_futures::JsFuture::from(
                        window.fetch_with_request(&request)
                    ).await;

                match resp {
                    Ok(val) => {
                        let response: Response = wasm_bindgen::JsCast::dyn_into(val).unwrap();
                        if response.ok() {
                            set_submitted.set(true);
                        } else {
                            set_error.set(Some("Submission failed. Please try again.".to_string()));
                        }
                    }
                    Err(_) => {
                        set_error.set(Some("Network error. Please try again.".to_string()));
                    }
                }

                set_loading.set(false);
            });
        }
    };

    view! {
        <div class="page contact-page">
            <div class="page-header">
                <p class="page-eyebrow">"~/contact"</p>
                <h1 class="page-title">"Get in Touch"</h1>
                <p class="page-subtitle">
                    "Have a project in mind, a role to fill, or just want to talk Rust? I'm all ears."
                </p>
            </div>

            <div class="contact-grid">
                <div class="contact-info">
                    <div class="contact-item">
                        <span class="contact-label">"// Location"</span>
                        <span class="contact-value">"Nairobi, Kenya"</span>
                    </div>
                    <div class="contact-item">
                        <span class="contact-label">"// GitHub"</span>
                        <a href="https://github.com/M0imaritim" target="_blank" rel="noopener" class="contact-link">
                            "github.com/M0imaritim"
                        </a>
                    </div>
                    <div class="contact-item">
                        <span class="contact-label">"// LinkedIn"</span>
                        <a href="https://www.linkedin.com/in/langat-moimaritim/" target="_blank" rel="noopener" class="contact-link">
                            "langat-moimaritim"
                        </a>
                    </div>
                    <div class="contact-availability">
                        <span class="avail-dot"></span>
                        "Available for freelance & full-time roles"
                    </div>
                </div>

                <div class="contact-form-wrap">
                    {move || if submitted.get() {
                        view! {
                            <div class="form-success">
                                <span class="success-icon">"✓"</span>
                                <h3>"Message received."</h3>
                                <p>"I'll get back to you soon."</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <form class="contact-form" on:submit=on_submit>
                                {move || error.get().map(|e| view! {
                                    <div class="form-error">{e}</div>
                                })}
                                <div class="form-field">
                                    <label class="form-label" for="name">"Name"</label>
                                    <input
                                        id="name"
                                        class="form-input"
                                        type="text"
                                        placeholder="Ada Lovelace"
                                        prop:value=name
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="form-field">
                                    <label class="form-label" for="email">"Email"</label>
                                    <input
                                        id="email"
                                        class="form-input"
                                        type="email"
                                        placeholder="ada@example.com"
                                        prop:value=email
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="form-field">
                                    <label class="form-label" for="message">"Message"</label>
                                    <textarea
                                        id="message"
                                        class="form-textarea"
                                        rows="6"
                                        placeholder="Tell me about your project..."
                                        prop:value=message
                                        on:input=move |ev| set_message.set(event_target_value(&ev))
                                    ></textarea>
                                </div>
                                <button
                                    type="submit"
                                    class="btn btn-primary btn-full"
                                    disabled=move || loading.get()
                                >
                                    {move || if loading.get() { "Sending..." } else { "Send Message →" }}
                                </button>
                            </form>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}