use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (submitted, set_submitted) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

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
        // TODO: wire to Axum server action for actual email sending (Resend / SendGrid)
        set_submitted.set(true);
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
                    <div class="contact-item">
                        <span class="contact-label">"// Twitter / X"</span>
                        <a href="https://twitter.com/Chum_code" target="_blank" rel="noopener" class="contact-link">
                            "@Chum_code"
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
                                <button type="submit" class="btn btn-primary btn-full">
                                    "Send Message →"
                                </button>
                            </form>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}