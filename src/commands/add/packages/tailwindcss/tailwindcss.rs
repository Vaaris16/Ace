use crate::frameworks::frameworks::Frameworks;

pub fn get_tailwind_instructions(framework: Frameworks) -> &'static str {
    match framework {
        Frameworks::React => {
            include_str!("instructions/tailwindcss_react_instructions.toml")
        }
        Frameworks::Angular => {
            include_str!("instructions/tailwindcss_angular_instructions.toml")
        }
        Frameworks::Vue => {
            include_str!("instructions/tailwindcss_vue_instructions.toml")
        }
        Frameworks::Svelte => {
            include_str!("instructions/tailwindcss_svelte_instructions.toml")
        }
        Frameworks::Solid => {
            include_str!("instructions/tailwindcss_solid_instructions.toml")
        }
        Frameworks::Preact => {
            include_str!("instructions/tailwindcss_preact_instructions.toml")
        }
        Frameworks::Qwik => {
            include_str!("instructions/tailwindcss_qwik_instructions.toml")
        }
        Frameworks::Lit => {
            include_str!("instructions/tailwindcss_lit_instructions.toml")
        }
        Frameworks::Ember => {
            include_str!("instructions/tailwindcss_ember_instructions.toml")
        }
        Frameworks::Marko => {
            include_str!("instructions/tailwindcss_marko_instructions.toml")
        }
        Frameworks::Vanilla => {
            include_str!("instructions/tailwindcss_vanilla_instructions.toml")
        }
    }
}
