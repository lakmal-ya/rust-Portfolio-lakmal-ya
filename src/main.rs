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
    // Cache-busting: stamp asset URLs with a hash of their contents so
    // browsers fetch the new file whenever it changes.
    let css_v = asset_hash("static/css/style.css");
    let js_v  = asset_hash("static/js/main.js");

    let html = HTML_TEMPLATE
        .replace("{{CSS_VERSION}}", &css_v)
        .replace("{{JS_VERSION}}",  &js_v)
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

fn asset_hash(path: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let bytes = fs::read(path).unwrap_or_default();
    let mut h = DefaultHasher::new();
    bytes.hash(&mut h);
    format!("{:x}", h.finish())
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
        ("Observability & Monitoring", &[
            ("Prometheus", "#E6522C"), ("Grafana", "#F46800"), ("Loki", "#F2CC0C"),
            ("Datadog", "#632CA6"), ("New Relic", "#1CE783"), ("ELK", "#00BFB3"),
            ("Azure Monitor / KQL", "#0078D4"), ("CloudWatch", "#FF9900"),
            ("Icinga2", "#06062C"), ("Slack Alerting", "#E01E5A"),
        ]),
        ("Incident & Problem Management", &[
            ("Incident Investigation", "#EF4444"), ("Root-Cause Analysis", "#F97316"),
            ("Runbooks", "#10b981"), ("Rollback Plans", "#3B82F6"),
            ("Change-Freeze Windows", "#8B5CF6"), ("Client Communication", "#14B8A6"),
        ]),
        ("Reliability & DR", &[
            ("Multi-Region (Cloudflare)", "#F38020"), ("Cross-Region DB Replication", "#0078D4"),
            ("HPA", "#326CE5"), ("Karpenter", "#FF9900"),
            ("Zero-Downtime Deployments", "#10b981"), ("Backup & Recovery", "#6366F1"),
        ]),
        ("Linux & Systems", &[
            ("RHEL", "#EE0000"), ("Ubuntu", "#E95420"), ("Windows Server", "#0078D4"),
            ("Apache", "#D22128"), ("Nginx", "#009639"), ("DNS / DHCP", "#64748B"),
            ("Load Balancing", "#0EA5E9"), ("SSL/TLS", "#22C55E"), ("VPN", "#A855F7"),
        ]),
        ("Automation & IaC", &[
            ("Bash", "#4EAA25"), ("Python", "#3776AB"), ("PowerShell", "#5391FE"),
            ("Terraform", "#7B42BC"), ("Ansible", "#EE0000"), ("Azure DevOps Pipelines", "#0078D4"),
        ]),
        ("Containers & CI/CD", &[
            ("Kubernetes (AKS, EKS)", "#326CE5"), ("Docker", "#2496ED"), ("Helm", "#0F1689"),
            ("ArgoCD", "#EF7B4D"), ("GitHub Actions", "#10b981"), ("Jenkins", "#D24939"),
            ("Azure DevOps", "#0078D4"), ("Trivy", "#1904DA"),
        ]),
        ("Cloud", &[
            ("AWS", "#FF9900"), ("EKS", "#FF9900"), ("ECS", "#FF9900"), ("EC2", "#FF9900"),
            ("VPC", "#FF9900"), ("ALB", "#FF9900"), ("RDS", "#FF9900"), ("S3", "#FF9900"),
            ("IAM", "#FF9900"), ("KMS", "#FF9900"),
            ("Azure", "#0078D4"), ("AKS", "#0078D4"), ("Azure SQL", "#0078D4"),
            ("Front Door", "#0078D4"), ("Entra ID", "#0078D4"), ("GCP", "#4285F4"),
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
            company:  "Sitecore",
            period:   "Aug 2025 - Present",
            location: "Sri Lanka",
            bullets: &[
                "<strong>Led the incident investigation</strong> of a live bot attack against an Azure Front Door-fronted application using KQL and Log Analytics. Traced the impact to rotating-IP traffic, a caching misconfiguration that sent every request to the origin, and HPA scaling lag, then delivered findings and mitigation steps to the client.",
                "Support a <strong>multi-region Azure setup for high availability and disaster recovery</strong>: application traffic load-balanced across two regions by Cloudflare, with critical databases replicated between regions.",
                "Led a Sitecore 10.3.1 to 10.4.1 platform upgrade across test and production on AKS and Azure SQL, including a SolrCloud migration from 8.11.2 to 9.8.1. Wrote and ran a <strong>21-step runbook with rollback points</strong> and content-freeze windows, coordinated across infrastructure and client teams.",
                "Retired a legacy ELK logging stack (Elastic Cloud, Filebeat, Logstash) after mapping its dependencies, <strong>consolidating monitoring on Azure Monitor and Datadog</strong> and cutting logging costs without losing visibility.",
                "Automated SolrCloud and ZooKeeper operations with PowerShell, including alias-based zero-downtime reindexing, and hardened the clusters with SSL/HTTPS.",
                "<strong>Automated after-hours shutdown of non-production AKS clusters and VMs</strong> with custom scripts run from scheduled Azure DevOps pipelines under a service principal. Environments stop outside working hours and start again before the workday, cutting idle compute spend.",
            ],
        },
        Job {
            role:     "Consultant - Cloud Ops",
            company:  "EY",
            period:   "Jan 2023 - Jul 2025",
            location: "Sri Lanka",
            bullets: &[
                "Set up <strong>Azure alerting</strong> that improved system reliability and incident response times.",
                "Built and maintained Terraform infrastructure as code to standardize and speed up AWS provisioning.",
                "Automated CI/CD with Jenkins, ArgoCD, and Bash, reducing deployment time by around 30%.",
                "Ran proof-of-concept builds for VMs and VNets to de-risk infrastructure ahead of full rollout.",
            ],
        },
        Job {
            role:     "DevOps Engineer",
            company:  "KeenEye",
            period:   "Jul 2022 - Dec 2022",
            location: "Sri Lanka",
            bullets: &[
                "<strong>Built a GitOps platform on AWS EKS from scratch</strong> for a social media marketing platform: Terraform-provisioned infrastructure, ArgoCD ApplicationSets deploying 7+ independent microservices, Traefik ingress with per-service middleware, Karpenter node autoscaling, and Prometheus, Grafana, and Loki with S3-backed logs. Every workload authenticated through <strong>IRSA with zero static AWS credentials</strong> in the cluster.",
                "Tuned application performance and <strong>monitoring</strong> to reduce downtime, and ran zero-downtime deployments with Helm and ArgoCD.",
                "Administered PostgreSQL, MySQL, and MongoDB, including <strong>backup and recovery</strong>, roles and access, and monitoring.",
            ],
        },
        Job {
            role:     "DevOps Engineer",
            company:  "EFutures",
            period:   "Aug 2021 - Jul 2022",
            location: "Sri Lanka",
            bullets: &[
                "Monitored and maintained AWS services (S3, EC2, RDS, VPC) to keep systems available, and ran Jenkins, Docker, and Git delivery for product and QA teams.",
                "Automated routine operations tasks with scripts, reducing manual work.",
            ],
        },
        Job {
            role:     "Associate System Administrator / System Administrator",
            company:  "EFutures",
            period:   "May 2019 - May 2021",
            location: "Sri Lanka",
            bullets: &[
                "Administered <strong>on-premise Linux and Windows servers</strong> alongside AWS, Azure, and GCP environments.",
                "Configured Apache and Nginx web servers, and monitored MySQL, MongoDB, and services with <strong>Icinga2</strong>.",
            ],
        },
        Job {
            role:     "Hardware Engineer",
            company:  "State Pharmaceutical Corporation",
            period:   "Oct 2015 - Dec 2016",
            location: "Sri Lanka",
            bullets: &[
                "Provided IT support for head office and branch users, maintaining computers and networks and resolving hardware and software issues.",
                "Assisted with deploying on-premise <strong>DHCP, DNS, and Microsoft Exchange</strong> servers.",
                "Assisted with configuring <strong>Cisco switches and routers</strong> to connect branch office networks.",
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
        CertData { icon: "&#x1F3C6;", name: "Microsoft Certified: Azure Fundamentals", issuer: "Microsoft", year: "2025" },
        CertData { icon: "&#x1F4DC;", name: "LFS169: Introduction to GitOps", issuer: "The Linux Foundation", year: "Completed" },
        CertData { icon: "&#x1F947;", name: "HashiCorp Certified: Terraform Associate (002)", issuer: "HashiCorp", year: "2023 &middot; Expired" },
        CertData { icon: "&#x1F947;", name: "AWS Certified Solutions Architect - Associate", issuer: "Amazon Web Services", year: "2022 &middot; Expired" },
        CertData { icon: "&#x1F3C6;", name: "Red Hat Certified System Administrator (RHCSA)", issuer: "Red Hat", year: "2019 &middot; Expired" },
        CertData { icon: "&#x1F947;", name: "Cisco Certified Network Associate (CCNA)", issuer: "Cisco", year: "2019 &middot; Expired" },
        CertData { icon: "&#x1F947;", name: "Google Cloud Platform Fundamentals: Core Infrastructure", issuer: "Google Cloud", year: "2019" },
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
