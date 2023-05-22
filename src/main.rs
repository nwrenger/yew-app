use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Page {
    #[at("/")]
    Index,
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contacts,
    #[at("/legals")]
    Legals,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(pages: Page) -> Html {
    match pages {
        Page::Index => html!{
            <div class="content" id="index">
                <div class="rounded-box">
                    <h2>{"Hi, there!"}</h2>
                    {"Welcome to my meaningless Website! It can't do anything now. Just go to my "} <a href="/#/projects" class="link">{"Projects"}</a>{" Section or go to my "}<a href="/#/about" class="link">{"About Me"}</a>{" Section. Lorem ipsum alum est. Mercator es ibus est. Inuria ia est."}<br/><br/>
                </div>
            </div>
        },
        Page::Projects => html!{
            <div class="content" id="projects">
                <div class="rounded-box">
                    <h2>{"My Projects"}</h2>
                    {"Currently, you can see all my Projects on my "}<a href="https://www.github.com/nwrenger" class="link">{"Github"}</a>{". Here may be something fun in the Future. Or is here something fun?"}<br/><br/>
                </div>
            </div>
        },
        Page::About => html!{
            <div class="content" id="about">
                <div class="rounded-box">
                    <h2>{"About Me"}</h2>
                    {"Hi there! My name is Nils and I'm a high school student who loves coding in my free time. I've always been interested in computers and technology, and programming has become a passion of mine over the years."}<br/><br/>
                    {"One of my favorite programming languages is Rust. I find Rust to be a powerful and efficient language that's well-suited for systems programming and other low-level tasks. I also appreciate Rust's focus on safety and security, which makes it a great choice for building reliable and robust software."}<br/><br/>
                    {"Aside from coding, I also enjoy playing my instruments: clarinet and saxophone. Music has always been a big part of my life, and I find that playing my instruments helps me unwind and stay creative."}<br/><br/>
                    {"When I'm not coding or playing music, I enjoy exploring new technologies and learning more about computer science. I'm particularly interested in areas like artificial intelligence, machine learning, and cybersecurity, and I hope to continue studying these fields in the future."}<br/><br/>
                    {"Thanks for stopping by and getting to know me a little better! If you have any questions or just want to chat about coding, music, or anything else, feel free to reach out."}<br/><br/>
                </div><br/>
                <div class="rounded-box">
                    <h2>{"About this Website"}</h2>
                    {"This Website was created and build with "}<a href="https://www.rust-lang.org/" class="link">{"Rust"}</a>{" and the "}<a href="https://yew.rs/" class="link">{"Yew Framework"}</a>{"."}<br/><br/>
                </div>
            </div>
        },
        Page::Contacts => html!{
            <div class="content" id="contacts">
                <div class="rounded-box">
                    <h2>{"Contact"}</h2>
                    {"You can contact me via my E-Mail "}<a href="mailto:nils@wrenger.net" class="link">{"nils@wrenger.net"}</a>{"."}<br/><br/>
                </div>
            </div>
        },
        Page::Legals => html!{
            <div class="content" id="legals">
                <div class="rounded-box">
                    <h2>{"Legals"}</h2>
                    <h2>{"English"}</h2>
                        {"This is my personal website, meant for my friends and as a personal hobby only. I don't make and don't intent to make money with it."}<br/><br/>
                        {"All the content represents my own opinion and is not guaranteed to be complete and correct."}<br/><br/>
                        {"I cannot provide any liability for any external links and their content found on this website."}<br/><br/>
                    <h2>{"Deutsch"}</h2>
                        {"Das ist meine private Webseite, die ausschließlich für meine Freunde und als persönliches Hobby gepflegt wird. Ich verdiene damit kein Geld, und habe es auch nicht vor."}<br/><br/>
                        {"Alle Inhalte repräsentieren meine persönliche Meinung und haben keinen Anspruch auf Vollständigkeit und Richtigkeit."}<br/><br/>
                        {"Haftung für den Inhalt externer Links ausgeschlossen."}<br/><br/>
                </div>
            </div>
        },
        Page::NotFound => html! {
            <div class="content" id="not_found">
              <div class="rounded-box">
                <h2>{"404 - Page not found"}</h2>
                {"Oh no! Your requested route was not Found!"}<br/><br/>
              </div>
            </div>
        }
    }
}
#[function_component]
fn App() -> Html {
    let toggle = use_state(|| false);
    let toggle_click = {
        let toggle = toggle.clone();
        move |_| {
            let bool = !*toggle;
            toggle.set(bool);
        }
    };
    let menu_button_class = if *toggle { "menu-button active" } else { "menu-button" };

    html! {
        <body>
            <header>
                <nav>
                    <div class="tabs" id="menu">
                        <li><button class={menu_button_class} onclick={toggle_click}>{ "Menu" }</button></li>
                    </div>
                    <div>
                        <ul class="tabs" id="tabs">
                            <li><a href="/" class="header-link">{ "Home" }</a></li>
                            <li><a href="/projects" class="header-link">{ "Projects" }</a></li>
                            <li><a href="/about" class="header-link">{ "About" }</a></li>
                            <li><a href="/contact" class="header-link">{ "Contacts" }</a></li>
                            <li><a href="/legals" class="header-link">{ "Legals" }</a></li>
                        </ul>
                    </div>
                    <div>
                    {if *toggle { html! {
                        <ul class="menu-tabs" id="menu-tabs">
                                <li><a href="/" class="header-link">{ "Home" }</a></li>
                                <li><a href="/projects" class="header-link">{ "Projects" }</a></li>
                                <li><a href="/about" class="header-link">{ "About" }</a></li>
                                <li><a href="/contact" class="header-link">{ "Contacts" }</a></li>
                                <li><a href="/legals" class="header-link">{ "Legals" }</a></li>                    
                        </ul>
                    }} else {html! {}}}
                    </div>
                    <div class="github-link-container">
                    <a href="https://github.com/nwrenger/website" target="_blank" rel="noreferrer" class="github-link">
                        { "Github" } <img src="https://raw.githubusercontent.com/nwrenger/nwrenger/master/external-link.svg" alt="github" width="12" height="12"/>
                    </a>
                    </div>
                </nav>
            </header>
            <BrowserRouter>
                <Switch<Page> render={switch} />
            </BrowserRouter>
        </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}