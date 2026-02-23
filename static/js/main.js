/* ===================================================
   Dulanjana Lakmal Portfolio - JavaScript
   Typing animation, scroll reveals, navbar behavior
   =================================================== */

// Signal to CSS that JS is running (enables opacity:0 animations)
document.documentElement.classList.add('js-ready');

/* ---- Typing terminal animation ---- */
const typingCmd = document.getElementById('typing-cmd');
const termOutput = document.getElementById('terminal-output');

const terminalSequences = [
    {
        cmd: 'whoami --verbose',
        output: `<div><span class="key">name</span>: <span class="str">"Dulanjana Lakmal"</span></div>
<div><span class="key">role</span>: <span class="str">"DevOps Engineer"</span></div>
<div><span class="key">experience</span>: <span class="num">6</span>+ years</div>
<div><span class="key">location</span>: <span class="str">"\uD83C\uDDF1\uD83C\uDDF0 Sri Lanka"</span></div>
<div><span class="key">status</span>: <span class="str">"open_to_work: true"</span></div>`,
    },
    {
        cmd: 'kubectl get skills --all',
        output: `<div><span class="comment"># Cloud & DevOps:</span></div>
<div><span class="key">AWS</span>    <span class="key">Kubernetes</span>   <span class="key">Terraform</span></div>
<div><span class="key">Docker</span> <span class="key">GitHub Actions</span> <span class="key">ArgoCD</span></div>
<div><span class="comment"># Languages:</span></div>
<div><span class="str">Python</span> <span class="str">Bash</span> <span class="str">Go</span> <span class="str">Rust</span></div>`,
    },
    {
        cmd: 'aws sts get-caller-identity',
        output: `<div><span class="key">Account</span>: <span class="num">************</span></div>
<div><span class="key">Arn</span>: <span class="str">"arn:aws:iam::*:user/dulanjana"</span></div>
<div><span class="key">UserId</span>: <span class="str">"devops-engineer"</span></div>
<div><span class="comment"># AWS certified ✓</span></div>`,
    },
];

let seqIndex = 0;

function typeText(text, el, speed, callback) {
    let i = 0;
    el.textContent = '';
    const interval = setInterval(() => {
        el.textContent += text[i];
        i++;
        if (i >= text.length) {
            clearInterval(interval);
            if (callback) callback();
        }
    }, speed);
}

function runTerminalSequence() {
    const seq = terminalSequences[seqIndex % terminalSequences.length];
    termOutput.innerHTML = '';

    typeText(seq.cmd, typingCmd, 60, () => {
        setTimeout(() => {
            termOutput.innerHTML = seq.output;
            termOutput.style.opacity = '0';
            termOutput.style.transition = 'opacity 0.4s ease';
            setTimeout(() => { termOutput.style.opacity = '1'; }, 50);

            setTimeout(() => {
                termOutput.style.opacity = '0';
                setTimeout(() => {
                    seqIndex++;
                    runTerminalSequence();
                }, 400);
            }, 3500);
        }, 400);
    });
}

// Start typing after 800ms
if (typingCmd) {
    setTimeout(runTerminalSequence, 800);
}

/* ---- Navbar scroll effect ---- */
const navbar = document.getElementById('navbar');

function handleNavbarScroll() {
    if (window.scrollY > 20) {
        navbar.classList.add('scrolled');
    } else {
        navbar.classList.remove('scrolled');
    }
}

window.addEventListener('scroll', handleNavbarScroll, { passive: true });

/* ---- Active nav link highlighting ---- */
const navLinks = document.querySelectorAll('.nav-link');
const sections = document.querySelectorAll('section[id]');

function highlightNavLink() {
    const scrollY = window.scrollY + 80;
    sections.forEach(section => {
        const sectionTop = section.offsetTop;
        const sectionHeight = section.offsetHeight;
        const sectionId = section.getAttribute('id');

        if (scrollY >= sectionTop && scrollY < sectionTop + sectionHeight) {
            navLinks.forEach(link => link.classList.remove('active'));
            const active = document.querySelector(`.nav-link[href="#${sectionId}"]`);
            if (active) active.classList.add('active');
        }
    });
}

window.addEventListener('scroll', highlightNavLink, { passive: true });

/* Inject active nav link style */
const styleEl = document.createElement('style');
styleEl.textContent = '.nav-link.active { color: var(--accent) !important; }';
document.head.appendChild(styleEl);

/* ---- Mobile nav toggle ---- */
const navToggle = document.getElementById('nav-toggle');
const navLinksEl = document.getElementById('nav-links');

if (navToggle && navLinksEl) {
    navToggle.addEventListener('click', () => {
        navLinksEl.classList.toggle('open');
    });

    // Close on link click
    navLinksEl.querySelectorAll('a').forEach(link => {
        link.addEventListener('click', () => navLinksEl.classList.remove('open'));
    });
}

/* ---- Scroll Reveal (Intersection Observer) ---- */
const revealObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('visible');
        }
    });
}, {
    threshold: 0.12,
    rootMargin: '0px 0px -40px 0px',
});

/* ---- Add reveal classes to section content ---- */
function addRevealClasses() {
    // Section headers
    document.querySelectorAll('.section-header').forEach(el => el.classList.add('reveal'));

    // Cards and grid items — group them
    document.querySelectorAll('.skills-grid').forEach(el => el.classList.add('reveal-group'));
    document.querySelectorAll('.projects-grid').forEach(el => el.classList.add('reveal-group'));
    document.querySelectorAll('.certs-grid').forEach(el => el.classList.add('reveal-group'));

    // Timeline items
    document.querySelectorAll('.timeline-item').forEach((el, i) => {
        el.classList.add('reveal');
        el.style.transitionDelay = `${i * 0.1}s`;
    });

    // About cards
    document.querySelectorAll('.about-grid > *, .contact-content > *').forEach((el, i) => {
        el.classList.add('reveal');
        el.style.transitionDelay = `${i * 0.15}s`;
    });

    // Social cards
    document.querySelectorAll('.social-links').forEach(el => el.classList.add('reveal-group'));

    // Start observing all revealed elements
    document.querySelectorAll('.reveal, .reveal-group').forEach(el => {
        revealObserver.observe(el);
    });

    // Immediately mark everything currently visible as .visible
    document.querySelectorAll('.reveal, .reveal-group').forEach(el => {
        const rect = el.getBoundingClientRect();
        if (rect.top < window.innerHeight) {
            el.classList.add('visible');
        }
    });
}

addRevealClasses();

/* ---- Smooth scroll for all anchor links ---- */
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', (e) => {
        e.preventDefault();
        const target = document.querySelector(anchor.getAttribute('href'));
        if (target) {
            const offset = 80;
            window.scrollTo({
                top: target.offsetTop - offset,
                behavior: 'smooth',
            });
        }
    });
});

/* ---- Skill badge color on hover ---- */
document.querySelectorAll('.skill-badge').forEach(badge => {
    const accentColor = badge.style.getPropertyValue('--accent');
    badge.addEventListener('mouseenter', () => {
        badge.style.borderColor = accentColor;
        badge.style.color = accentColor;
        badge.style.boxShadow = `0 0 12px ${accentColor}25`;
    });
    badge.addEventListener('mouseleave', () => {
        badge.style.borderColor = '';
        badge.style.color = '';
        badge.style.boxShadow = '';
    });
});

/* ---- Copy email on click ---- */
const emailBtns = document.querySelectorAll('a[href^="mailto"]');
emailBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        const email = 'dulanjanalakmal@gmail.com';
        if (navigator.clipboard) {
            navigator.clipboard.writeText(email).catch(() => { });
        }
    });
});

console.log('%c🦀 Portfolio by Dulanjana Lakmal', 'color:#10b981;font-family:monospace;font-size:16px;font-weight:bold;');
console.log('%cBuilt with Rust + Deployed on AWS S3 + CloudFront', 'color:#71717a;font-family:monospace;font-size:12px;');
