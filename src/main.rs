use std::fs;
use std::path::Path;

// Embed the HTML template at compile time - no runtime file I/O needed
static HTML_TEMPLATE: &str = include_str!("../templates/index.html");

fn main() {
    println!("Building Dulanjana Lakmal Portfolio...");

    let dist_dir = Path::new("dist");
    let dist_css_dir = dist_dir.join("css");
    let dist_js_dir = dist_dir.join("js");

    for dir in &[dist_dir, &dist_css_dir, &dist_js_dir] {
        fs::create_dir_all(dir).expect("Failed to create directory");
    }

    // Copy static assets
    copy_file("static/css/style.css", &dist_dir.join("css/style.css"));
    copy_file("static/js/main.js",    &dist_dir.join("js/main.js"));

    // Build HTML by substituting placeholders in template
    let html = HTML_TEMPLATE
        .replace("{{SKILLS}}",       &render_skills())
        .replace("{{EXPERIENCE}}",   &render_experience())
        .replace("{{PROJECTS}}",     &render_projects())
        .replace("{{CERTS}}",        &render_certs());

    fs::write(dist_dir.join("index.html"), &html)
        .expect("Failed to write dist/index.html");

    println!("Portfolio built successfully!");
    println!("Output: ./dist/index.html");
}

fn copy_file(src: &str, dst: &Path) {
    let src_path = Path::new(src);
    if src_path.exists() {
        fs::copy(src_path, dst).expect("Failed to copy file");
    }
}

// ── Skill badges ──────────────────────────────────────────────────────────────

fn render_skills() -> String {
    let categories: &[(&str, &[(&str, &str)])] = &[
        ("Cloud & Infrastructure", &[
            ("AWS", "#FF9900"), ("Azure", "#0078D4"), ("GCP", "#4285F4"),
            ("EC2", "#FF9900"), ("S3", "#FF9900"), ("CloudFront", "#FF9900"),
            ("RDS", "#FF9900"), ("VPC", "#FF9900"), ("IAM", "#FF9900"),
        ]),
        ("DevOps & Infrastructure as Code", &[
            ("Terraform", "#7B42BC"), ("CloudFormation", "#FF9900"), ("Ansible", "#EE0000"),
            ("Kubernetes", "#326CE5"), ("Docker", "#2496ED"), ("Linux", "#FCC624"),
        ]),
        ("CI/CD & Automation", &[
            ("Jenkins", "#D24939"), ("ArgoCD", "#EF7B4D"), ("GitHub Actions", "#10b981"),
            ("Git", "#F05032"), ("Bash Scripting", "#4EAA25"),
        ]),
        ("Monitoring & Solutions", &[
            ("Zabbix", "#E42528"), ("Prometheus", "#E6522C"), ("Grafana", "#F46800"),
            ("CloudWatch", "#FF9900"), ("REST APIs", "#4CAF50"),
        ]),
        ("Core Competencies", &[
            ("Solution Architecture", "#9C27B0"), ("DevOps Engineering", "#FF6F00"),
            ("System Administration", "#2196F3"), ("Cloud Operations", "#009688"),
            ("RHCSA", "#CC0000"),
        ]),
        ("Languages & Database", &[
            ("Bash", "#4EAA25"), ("Python", "#3776AB"), ("MySQL", "#00758F"),
            ("MongoDB", "#13AA52"), ("Windows Server", "#0078D4"),
        ]),
    ];

    let mut out = String::new();
    for (category, items) in categories {
        let badges: String = items.iter().map(|(name, color)| {
            format!(
                "<span class=\"skill-badge\" style=\"--accent:{color};\">{name}</span>",
                color = color, name = name
            )
        }).collect::<Vec<_>>().join(" ");

        out.push_str(&format!(
            "<div class=\"skill-category\">\
               <h3 class=\"skill-category-title\">{cat}</h3>\
               <div class=\"skill-badges\">{badges}</div>\
             </div>\n",
            cat    = category,
            badges = badges
        ));
    }
    out
}

// ── Experience timeline ───────────────────────────────────────────────────────

struct Job {
    role:     &'static str,
    company:  &'static str,
    period:   &'static str,
    location: &'static str,
    bullets:  &'static [&'static str],
}

fn render_experience() -> String {
    let jobs: &[Job] = &[
        Job {
            role:     "Senior Cloud Solutions Engineer",
            company:  "IGT1 Lanka - Full-time",
            period:   "August 2025 - Present",
            location: "Colombo, Sri Lanka",
            bullets: &[
                "Design and deploy scalable cloud infrastructure solutions leveraging AWS, Azure, and GCP.",
                "Build Infrastructure as Code solutions using Terraform and CloudFormation for enterprise clients.",
                "Implement CI/CD pipelines and DevOps best practices to streamline deployment processes.",
                "Lead cloud architecture reviews and provide technical guidance on solution design.",
                "Manage and optimize cloud costs while maintaining security and performance standards.",
            ],
        },
        Job {
            role:     "Cloud Operations Engineer",
            company:  "EY - Full-time",
            period:   "January 2023 - July 2025",
            location: "Sri Lanka",
            bullets: &[
                "Developed Infrastructure as Code (IAC) with Terraform, ensuring efficient AWS resource utilization tailored to project needs.",
                "Automated deployment processes using Jenkins and ArgoCD, significantly reducing manual errors and improving operational efficiency.",
                "Led POC initiatives for VMs and VNets, validating setups before deployment, which decreased deployment errors and increased reliability.",
                "Managed multi-cloud infrastructure across AWS and Azure, optimizing for cost and performance.",
                "Implemented monitoring and alerting solutions reducing mean time to resolution (MTTR).",
            ],
        },
        Job {
            role:     "DevOps Engineer",
            company:  "KeenEye - Full-time",
            period:   "July 2022 - December 2022",
            location: "Western Province, Sri Lanka",
            bullets: &[
                "Built and maintained robust CI/CD pipelines, enhancing code integration and deployment efficiency.",
                "Developed infrastructure as code (IAC) using Terraform and Ansible, streamlining infrastructure management.",
                "Automated deployment processes using Jenkins, significantly reducing release cycle times.",
                "Proactively monitored application performance and infrastructure health, leading to swift issue resolution and reduced downtime.",
            ],
        },
        Job {
            role:     "DevOps Engineer",
            company:  "EFutures Private Limited - Full-time",
            period:   "August 2021 - July 2022",
            location: "Sri Lanka",
            bullets: &[
                "Collaborated with product development and QA teams to ensure delivery of high-quality software products.",
                "Maintained high system availability using Jenkins, Docker, and Git for efficient DevOps operations.",
                "Configured and managed AWS services such as S3, EC2, RDS, and VPC for optimal cloud performance.",
                "Automated system tasks with custom Bash and Python scripts, enhancing operational efficiency.",
                "Increased system uptime by maintaining and monitoring AWS services with proactive alerting.",
            ],
        },
        Job {
            role:     "Associate System Administrator",
            company:  "EFutures Private Limited - Full-time",
            period:   "October 2019 - May 2021",
            location: "Sri Lanka",
            bullets: &[
                "Managed AWS cloud services including EC2, S3, RDS, and ELB, ensuring robust cloud operations.",
                "Handled MySQL, Icinga2, and MongoDB for efficient database management and monitoring.",
                "Installed and configured web servers like Apache and Nginx for secure and efficient operations.",
                "Optimized cloud resource usage through effective AWS service management and cost optimization.",
                "Improved database performance and uptime with proactive monitoring and management strategies.",
            ],
        },
        Job {
            role:     "System Administrator",
            company:  "EFutures Private Limited - Full-time",
            period:   "May 2019 - October 2019",
            location: "Sri Lanka",
            bullets: &[
                "Managed cloud platforms including AWS, GCP, and Azure, ensuring optimal performance.",
                "Administered Linux and Windows servers, maintaining system stability and performance.",
                "Worked with version control systems to ensure efficient and organized code management.",
                "Enhanced web server performance by optimizing configurations and reducing incidents.",
            ],
        },
        Job {
            role:     "Hardware Engineer",
            company:  "State Pharmaceutical Corporation - Full-time",
            period:   "October 2015 - December 2016",
            location: "Sri Lanka",
            bullets: &[
                "Installed and maintained computer systems and networks to ensure smooth operations.",
                "Efficiently diagnosed and resolved hardware and software issues.",
                "Troubleshot ADSL routers and printers to maintain network and device functionality.",
                "Improved network reliability through effective troubleshooting and proactive maintenance.",
            ],
        },
    ];

    let mut out = String::new();
    for job in jobs {
        let bullets: String = job.bullets.iter()
            .map(|b| format!("<li>{}</li>", b))
            .collect::<Vec<_>>().join("\n");

        out.push_str(&format!(
            "<div class=\"timeline-item\">\
               <div class=\"timeline-dot\"></div>\
               <div class=\"timeline-card card\">\
                 <div class=\"timeline-header\">\
                   <div>\
                     <h3 class=\"timeline-role\">{role}</h3>\
                     <p class=\"timeline-company\">{company}</p>\
                   </div>\
                   <div class=\"timeline-meta\">\
                     <span class=\"timeline-date\">{period}</span>\
                     <span class=\"timeline-location\">{location}</span>\
                   </div>\
                 </div>\
                 <ul class=\"timeline-bullets\">{bullets}</ul>\
               </div>\
             </div>\n",
            role     = job.role,
            company  = job.company,
            period   = job.period,
            location = job.location,
            bullets  = bullets
        ));
    }
    out
}

// ── Projects ──────────────────────────────────────────────────────────────────

struct ProjectData {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    url: &'static str,
}

fn render_projects() -> String {
    let projects: &[ProjectData] = &[
        ProjectData {
            icon: "&#x2601;",
            title: "Complete Cloud Infrastructure Solutions",
            description: "Architected and deployed enterprise-grade cloud infrastructure on AWS, Azure, and GCP encompassing IAC with Terraform, automated deployments, and scalable architectures.",
            tags: &["AWS", "Azure", "GCP", "Terraform", "CloudFormation"],
            url: "https://www.linkedin.com/in/dulanjanalakmal",
        },
        ProjectData {
            icon: "&#x2699;",
            title: "DevOps Automation & CI/CD Pipelines",
            description: "Implemented robust CI/CD pipelines using Jenkins and ArgoCD with automated testing, containerization, and multi-environment deployments reducing release cycles significantly.",
            tags: &["Jenkins", "ArgoCD", "Docker", "Automation", "Bash"],
            url: "https://www.linkedin.com/in/dulanjanalakmal",
        },
        ProjectData {
            icon: "&#x1F680;",
            title: "Infrastructure as Code Framework",
            description: "Developed reusable Terraform and CloudFormation modules for rapid infrastructure provisioning, ensuring consistency, security, and cost optimization across multiple projects.",
            tags: &["Terraform", "CloudFormation", "Ansible", "Infrastructure as Code", "AWS"],
            url: "https://www.linkedin.com/in/dulanjanalakmal",
        },
        ProjectData {
            icon: "&#x1F4CA;",
            title: "Cloud Monitoring & Observability Solutions",
            description: "Implemented comprehensive monitoring solutions using Zabbix, Prometheus, Grafana, and CloudWatch for real-time visibility and proactive alerting across multi-cloud environments.",
            tags: &["Zabbix", "Prometheus", "Grafana", "CloudWatch", "Monitoring"],
            url: "https://www.linkedin.com/in/dulanjanalakmal",
        },
        ProjectData {
            icon: "&#x1F512;",
            title: "Multi-Cloud Solution Architecture",
            description: "Designed and implemented scalable, secure, and cost-optimized solution architectures leveraging AWS, Azure, and GCP based on business requirements and best practices.",
            tags: &["Solution Architecture", "AWS", "Azure", "GCP", "Cloud Design"],
            url: "https://www.linkedin.com/in/dulanjanalakmal",
        },
        ProjectData {
            icon: "&#x1F980;",
            title: "Technical Content & Blogging",
            description: "Share insights on DevOps best practices, cloud optimization, and automation strategies as a medium-level tech article writer helping teams stay updated with industry trends.",
            tags: &["Technical Writing", "DevOps", "Cloud", "Medium", "Blogging"],
            url: "https://medium.com/@dulanjanalakmal",
        },
    ];

    let mut out = String::new();
    for p in projects {
        let tags: String = p.tags.iter()
            .map(|t| format!("<span class=\"project-tag\">{}</span>", t))
            .collect::<Vec<_>>().join(" ");

        out.push_str(&format!(
            "<div class=\"project-card card hover-lift\">\
               <div class=\"project-header\">\
                 <span class=\"project-icon\">{icon}</span>\
                 <h3 class=\"project-title\">{title}</h3>\
               </div>\
               <p class=\"project-desc\">{desc}</p>\
               <div class=\"project-tags\">{tags}</div>\
               <a href=\"{url}\" target=\"_blank\" rel=\"noopener\" class=\"project-link\">View on GitHub &rarr;</a>\
             </div>\n",
            icon  = p.icon,
            title = p.title,
            desc  = p.description,
            tags  = tags,
            url   = p.url
        ));
    }
    out
}

// ── Certifications ────────────────────────────────────────────────────────────

struct CertData {
    icon:   &'static str,
    name:   &'static str,
    issuer: &'static str,
    year:   &'static str,
}

fn render_certs() -> String {
    let certs: &[CertData] = &[
        CertData { icon: "&#x1F3C6;", name: "RHCSA - Red Hat Certified System Administrator", issuer: "Red Hat", year: "2024" },
        CertData { icon: "&#x1F3C6;", name: "AWS Certified Cloud Practitioner", issuer: "Amazon Web Services", year: "2023" },
        CertData { icon: "&#x1F947;", name: "Microsoft Certified: Azure Fundamentals", issuer: "Microsoft", year: "2023" },
        CertData { icon: "&#x1F947;", name: "Google Cloud Platform Fundamentals: Core Infrastructure", issuer: "Google Cloud", year: "2023" },
        CertData { icon: "&#x1F947;", name: "LFS169: Introduction to GitOps", issuer: "Linux Foundation", year: "2023" },
    ];

    let mut out = String::new();
    for c in certs {
        out.push_str(&format!(
            "<div class=\"cert-card card hover-lift\">\
               <div class=\"cert-icon\">{icon}</div>\
               <div class=\"cert-info\">\
                 <h3 class=\"cert-name\">{name}</h3>\
                 <p class=\"cert-issuer\">{issuer}</p>\
                 <span class=\"cert-year\">{year}</span>\
               </div>\
             </div>\n",
            icon   = c.icon,
            name   = c.name,
            issuer = c.issuer,
            year   = c.year
        ));
    }
    out
}
