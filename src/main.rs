use std::fs;
use std::path::Path;

// Embed the HTML template at compile time - no runtime file I/O needed
static HTML_TEMPLATE: &str = include_str!("../templates/index.html");

fn main() {
    println!("Building Dulanjana Lakmal Portfolio...");

    let dist_dir = Path::new("dist");
    let dist_css_dir = dist_dir.join("css");
    let dist_js_dir = dist_dir.join("js");
    let dist_images_dir = dist_dir.join("images");

    for dir in &[dist_dir, &dist_css_dir, &dist_js_dir, &dist_images_dir] {
        fs::create_dir_all(dir).expect("Failed to create directory");
    }

    // Copy static assets
    copy_file("static/css/style.css", &dist_dir.join("css/style.css"));
    copy_file("static/js/main.js",    &dist_dir.join("js/main.js"));
    copy_file("static/images/avatar.jpeg", &dist_images_dir.join("avatar.jpeg"));

    // Build HTML by substituting placeholders in template
    let html = HTML_TEMPLATE
        .replace("{{SKILLS}}",       &render_skills())
        .replace("{{EXPERIENCE}}",   &render_experience())
        .replace("{{PROJECTS}}",     &render_projects())
        .replace("{{BLOG_ARTICLES}}", &render_blog_articles())
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
            icon: "&#x1F980;",
            title: "Rust Portfolio Website",
            description: "Personal portfolio built with Rust, featuring infrastructure as code with Terraform, deployed on AWS S3 with CloudFront CDN for optimal performance and modern design.",
            tags: &["Rust", "Terraform", "AWS", "S3", "CloudFront"],
            url: "https://github.com/lakmal-ya/rust-Portfolio-lakmal-ya",
        },
        ProjectData {
            icon: "&#x1F4CA;",
            title: "Azure Alerts Manager",
            description: "Python-based tool for managing Azure alerts at scale. Easily enable/disable both metric and log-based alerts with a single command for improved operational efficiency.",
            tags: &["Python", "Azure", "Alerts", "CLI", "Automation"],
            url: "https://github.com/lakmal-ya/azure-alerts-manager",
        },
        ProjectData {
            icon: "&#x1F680;",
            title: "EC2 Instances Start/Stop with Terraform & Lambda",
            description: "Automate AWS EC2 instance lifecycle management using Terraform Infrastructure as Code and AWS Lambda for cost optimization and dynamic resource management.",
            tags: &["Terraform", "AWS", "Lambda", "EC2", "IaC"],
            url: "https://github.com/lakmal-ya/EC2-Instances-Start-Stop-with-Terraform-Lambda",
        },
        ProjectData {
            icon: "&#x1F50F;",
            title: "AWS VPN Connection Terraform Module",
            description: "Reusable Terraform module for configuring AWS site-to-site VPN connections, enabling secure communication between on-premises and cloud infrastructure.",
            tags: &["Terraform", "AWS", "VPN", "Networking", "IaC"],
            url: "https://github.com/lakmal-ya/aws_vpn_connection_tf_module",
        },
        ProjectData {
            icon: "&#x1F4A7;",
            title: "Terraform Bitbucket Multi-Repo Manager",
            description: "Terraform configuration for managing multiple Bitbucket repositories, variables, and deployments within a single project for streamlined infrastructure management.",
            tags: &["Terraform", "Bitbucket", "IaC", "Repository Management", "HCL"],
            url: "https://github.com/lakmal-ya/terraform-bitbucket-multi-repo-manager",
        },
        ProjectData {
            icon: "&#x2693;",
            title: "Calcom on Kubernetes",
            description: "Deployment configuration for running Calcom scheduling application and PostgreSQL database on Kubernetes clusters for scalable calendar and scheduling solutions.",
            tags: &["Kubernetes", "Docker", "Calcom", "PostgreSQL", "K8s"],
            url: "https://github.com/lakmal-ya/calcom-k8s",
        },
        ProjectData {
            icon: "&#x1F4E6;",
            title: "ArgoCD GitOps",
            description: "GitOps-based deployment strategy using ArgoCD for continuous delivery, enabling declarative infrastructure and application management through version-controlled configurations.",
            tags: &["ArgoCD", "GitOps", "Kubernetes", "CI/CD", "DevOps"],
            url: "https://github.com/lakmal-ya/argocd-gitops",
        },
        ProjectData {
            icon: "&#x1F4A8;",
            title: "FossFLOW - Isometric Infrastructure Diagrams",
            description: "Create beautiful isometric infrastructure diagrams for visualizing cloud architecture. TypeScript-based tool for designing and exporting technical infrastructure diagrams.",
            tags: &["TypeScript", "Diagrams", "Infrastructure", "Visualization", "MIT Licensed"],
            url: "https://github.com/lakmal-ya/FossFLOW",
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

// ── Blog Articles ─────────────────────────────────────────────────────────────

fn render_blog_articles() -> String {
    let articles: &[(&str, &str, &str, &str, &str)] = &[
        (
            "Setting Up Modular Capability Plugins (MCP) with Amazon Q on Arch Linux",
            "Learn how to enhance Amazon Q with specialized MCPs for AWS architecture diagrams, documentation, and Terraform best practices on Arch Linux.",
            "https://blog.stackademic.com/setting-up-modular-capability-plugins-mcp-with-amazon-q-on-arch-linux-1933d11fc491",
            "May 14, 2025",
            "AWS & AI"
        ),
        (
            "Python Script to Manage Azure Alerts",
            "Automate Azure alert management with Python. Enable, disable, and manage both metric and log-based alerts at scale with a simple script.",
            "https://medium.com/@dulanjanalakmal/python-script-to-manage-azure-alerts-63f4c578930f",
            "Feb 11, 2025",
            "Azure & DevOps"
        ),
        (
            "Securing Linux Filesystems: Best Practices for DevOps Security",
            "Comprehensive guide on securing Linux filesystems including permissions, encryption, auditing, and DevOps-specific security practices.",
            "https://medium.com/@dulanjanalakmal/securing-linux-filesystems-best-practices-for-devops-security-9f57117fe30b",
            "Sep 3, 2024",
            "Linux Security"
        ),
        (
            "Handling Dynamic Routing in Amazon CloudFront with Lambda@Edge for a Next.js Application",
            "Master dynamic and static routing for Next.js applications deployed on AWS CloudFront using Lambda@Edge functions.",
            "https://blog.stackademic.com/handling-dynamic-routing-in-amazon-cloudfront-with-lambda-edge-for-a-next-js-application-a946887f4422",
            "Jul 22, 2024",
            "AWS & Next.js"
        ),
        (
            "Automating Website Deployment to AWS EC2 Using GitHub Actions",
            "Complete guide to automating website deployment to AWS EC2 using GitHub Actions workflows with rsync and SSH.",
            "https://medium.com/@dulanjanalakmal/automating-website-deployment-to-aws-ec2-using-github-actions-76997c52747a",
            "Dec 10, 2023",
            "CI/CD"
        ),
    ];

    let mut out = String::new();
    for (title, description, link, date, category) in articles {
        out.push_str(&format!(
            "<div class=\"blog-card card hover-lift\">\
               <div class=\"blog-header\">\
                 <span class=\"blog-source\">{}</span>\
                 <span class=\"blog-date\">{}</span>\
               </div>\
               <h3 class=\"blog-title\">{}</h3>\
               <p class=\"blog-desc\">{}</p>\
               <div class=\"blog-tags\">\
                 <span class=\"blog-tag\">DevOps</span>\
                 <span class=\"blog-tag\">Cloud Engineering</span>\
               </div>\
               <a href=\"{}\" target=\"_blank\" rel=\"noopener\" class=\"blog-link\">Read Article →</a>\
             </div>\n",
            category, date, title, description, link
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
