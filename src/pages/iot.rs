use leptos::prelude::*;

#[component]
pub fn IoTPage() -> impl IntoView {
    let focus_areas = vec![
        ("⚡", "Embedded Systems", "Designing circuits and programming microcontrollers — turning bare metal into working systems."),
        ("📡", "Sensor Integration", "Connecting sensors to collect real-world data: temperature, humidity, motion, environmental metrics."),
        ("☁", "IoT Cloud Platforms", "Sending, storing, and analysing sensor data in the cloud. Edge-to-cloud pipelines."),
        ("🤖", "Automation & Control", "Smart systems that respond to their environment and trigger actions without human intervention."),
    ];

    view! {
        <div class="page iot-page">
            <div class="page-header">
                <p class="page-eyebrow">"~/iot"</p>
                <h1 class="page-title">"Internet of Things"</h1>
                <p class="page-subtitle">
                    "Where code meets the physical world. Embedded systems, sensor networks, and the joy of blinking an LED at 3 AM."
                </p>
            </div>

            <section class="section">
                <h2 class="section-title">
                    <span class="section-num">"01"</span>
                    "Focus Areas"
                </h2>
                <div class="iot-grid">
                    {focus_areas.into_iter().map(|(icon, title, desc)| view! {
                        <div class="iot-card">
                            <span class="iot-icon">{icon}</span>
                            <h3 class="iot-title">{title}</h3>
                            <p class="iot-desc">{desc}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </section>

            <section class="section">
                <h2 class="section-title">
                    <span class="section-num">"02"</span>
                    "Projects"
                </h2>
                <div class="iot-projects">
                    <div class="iot-project-card">
                        <div class="iot-project-header">
                            <h3>"IoT Smart Home System"</h3>
                            <span class="badge badge-wip">"IN PROGRESS"</span>
                        </div>
                        <p>
                            "A home automation system using microcontrollers and sensor networks. "
                            "Controls and monitors environmental conditions, with cloud data ingestion and a dashboard."
                        </p>
                        <div class="iot-project-links">
                            <a
                                href="https://github.com/M0imaritim/SmartHomeSystems"
                                target="_blank"
                                rel="noopener"
                                class="btn btn-ghost"
                            >
                                "{ } Code"
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            <section class="section">
                <h2 class="section-title">
                    <span class="section-num">"03"</span>
                    "Certification"
                </h2>
                <div class="cert-card">
                    <div class="cert-info">
                        <p class="cert-issuer">"Certified IoT Professional"</p>
                        <p class="cert-desc">"Formal certification covering embedded systems, sensor networks, and IoT cloud platforms."</p>
                    </div>
                    <a
                        href="https://drive.google.com/file/d/1F1AepLcKeKu7G4XMAlY2YCEDGK4YkwgX/view"
                        target="_blank"
                        rel="noopener"
                        class="btn btn-primary"
                    >
                        "↗ View Certificate"
                    </a>
                </div>
            </section>
        </div>
    }
}
