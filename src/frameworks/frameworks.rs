use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Frameworks {
    Vanilla,
    Vue,
    React,
    Preact,
    Lit,
    Svelte,
    Solid,
    Ember,
    Qwik,
    Angular,
    Marko,
}

impl Frameworks {
    pub fn get_framework_str(self) -> &'static str {
        match self {
            Frameworks::Vanilla => "vanilla",
            Frameworks::Vue => "vue",
            Frameworks::React => "react",
            Frameworks::Preact => "preact",
            Frameworks::Lit => "lit",
            Frameworks::Svelte => "svelte",
            Frameworks::Solid => "solid",
            Frameworks::Ember => "ember",
            Frameworks::Qwik => "qwik",
            Frameworks::Angular => "angular",
            Frameworks::Marko => "marko",
        }
    }
}
